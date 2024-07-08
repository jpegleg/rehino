-use std::collections::HashMap;
use std::sync::Arc;
use std::cell::RefCell;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::error::Error;
use std::io::Cursor;

use uuid::Uuid;
use async_trait::async_trait;
use russh::*;
use russh_keys::*;
use tokio::sync::Mutex;
use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine as _};
use log::{Log, Record, Metadata, LevelFilter};

use russh::server::{Msg, Server as _, Session};
use russh_keys::key::SignatureHash;

struct SessionLogger {
    inner: Box<dyn Log>,
    session_counter: AtomicU64,
}

impl SessionLogger {
    fn new(inner: Box<dyn Log>) -> Self {
        SessionLogger {
            inner,
            session_counter: AtomicU64::new(0),
        }
    }

    fn new_session_id(&self) -> String {
        let counter = self.session_counter.fetch_add(1, Ordering::SeqCst);
        let uuid = Uuid::new_v4();
        format!("{}_{}", counter, uuid)
    }
}

thread_local! {
    static LOG_BUFFER: RefCell<String> = RefCell::new(String::with_capacity(1024));
}

impl Log for SessionLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.inner.enabled(metadata)
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let new_args = format!("{:?}", record);
            self.inner.log(&Record::builder()
                .args(format_args!("{}", new_args))
                .level(record.level())
                .target(record.target())
                .module_path(record.module_path())
                .file(record.file())
                .build());
        }
    }

    fn flush(&self) {
        self.inner.flush();
    }
}

#[derive(Clone)]
struct Server {
    clients: Arc<Mutex<HashMap<(usize, ChannelId), russh::server::Handle>>>,
    id: usize,
    client_ip: Option<SocketAddr>,
    session_id: String,
    logger: Arc<SessionLogger>,
}

impl Server {

    async fn post(&mut self, data: CryptoVec) {
        let mut clients = self.clients.lock().await;
        for ((id, channel), ref mut s) in clients.iter_mut() {
            if *id != self.id {
                let _ = s.data(*channel, data.clone()).await;
            }
        }
    }

}

impl server::Server for Server {
    type Handler = Self;
    fn new_client(&mut self, addr: Option<std::net::SocketAddr>) -> Self {
        let mut s = self.clone();
        s.id += 1;
        s.client_ip = addr;
        s.session_id = s.logger.new_session_id();

        log::debug!("New client connected - IP: {:?}, Session ID: {}", s.client_ip, s.session_id);

        s
    }
}

#[async_trait]
impl server::Handler for Server {
    type Error = anyhow::Error;

    async fn channel_open_session(
        &mut self,
        _channel: Channel<Msg>,
        _session: &mut Session,
    ) -> Result<bool, Self::Error> {
        log::debug!("Channel session opened {}", &self.session_id);
        Ok(true)
    }

    async fn auth_publickey(
        &mut self,
        username: &str,
        public_key: &key::PublicKey,
    ) -> Result<server::Auth, Self::Error> {
        let mut debug_buffer = Vec::new();
        if let Err(e) = public_key.write_debug_info(&mut debug_buffer) {
            log::warn!("Failed to write public key debug info: {}", e);
        } else {
            let debug_info = String::from_utf8_lossy(&debug_buffer);
            log::debug!(
                "{} - Public key info for user {}:\n{}",
                &self.session_id,
                username,
                debug_info
            );
        }

        Ok(server::Auth::Accept)
    }

    async fn data(
        &mut self,
        channel: ChannelId,
        data: &[u8],
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        let rawb = STANDARD_NO_PAD.encode(data);
        let dato = CryptoVec::from(format!("{:?}", &data));

        log::debug!("Received data - Session ID: {} - Data: {:?}", self.session_id, &dato);
        log::debug!("Received raw data - Session ID: {} - Raw B64: {:?}", self.session_id, &rawb);

        self.post(dato.clone()).await;
        session.data(channel, dato);
        Ok(())
    }

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>  {
    let env_logger = env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Debug)
        .build();

    let session_logger = Arc::new(SessionLogger::new(Box::new(env_logger)));
    log::set_boxed_logger(Box::new(session_logger.clone()))?;
    log::set_max_level(LevelFilter::Debug);

    let config = russh::server::Config {
        inactivity_timeout: Some(std::time::Duration::from_secs(3600)),
        auth_rejection_time: std::time::Duration::from_secs(121),
        auth_rejection_time_initial: Some(std::time::Duration::from_secs(0)),
        keys: vec![russh_keys::key::KeyPair::generate_rsa(2048, SignatureHash::SHA2_256).unwrap(), russh_keys::key::KeyPair::generate_ed25519().unwrap()],
        ..Default::default()
    };
    let config = Arc::new(config);

    let mut honey = Server {
        clients: Arc::new(Mutex::new(HashMap::new())),
        id: 0,
        client_ip: None,
        session_id: "".to_string(),
        logger: session_logger,
    };
    honey.run_on_address(config, ("0.0.0.0", 22)).await?;

    Ok(())
}
