# rehino - a russh honeypot

This fork of russh contains additional modification and tooling for the purpose of honeypot engineering, debugging, and more.

The program `rehino` is an ssh honeypot made from the russh echoserver example. It has been adapted to have verbose logging and data capture.

The russh has been modified to support customizing the ssh header packet that displays the version, as to spoof various versions.

The `rehino` program uses cargo patches to pull from the local version of russh rather than crates.io.

## Debugging every function in detail

The rehino honeypot debugs in an extremely verbose way, including each function call with source code file and line number.

Here is an example of some rehino log output:

```
[2024-07-09T02:24:19Z DEBUG rehino] Record { metadata: Metadata { level: Debug, target: "rehino" }, args: New client connected - IP: Some(192.168.1.22:53856), Session ID: 3_a38347f1-56e3-4bde-83d5-4eed5da57489, module_path: Some(Static("rehino")), file: Some(Static("rehino/src/main.rs")), line: Some(95) }
[2024-07-09T02:24:19Z DEBUG russh::ssh_read] Record { metadata: Metadata { level: Debug, target: "russh::ssh_read" }, args: read_ssh_id: reading, module_path: Some(Static("russh::ssh_read")), file: Some(Static("/home/honey/workspace/rehino/russh/src/ssh_read.rs")), line: Some(123) }
[2024-07-09T02:24:19Z DEBUG russh::ssh_read] Record { metadata: Metadata { level: Debug, target: "russh::ssh_read" }, args: read 32, module_path: Some(Static("russh::ssh_read")), file: Some(Static("/home/honey/workspace/rehino/russh/src/ssh_read.rs")), line: Some(127) }
[2024-07-09T02:24:19Z DEBUG russh::ssh_read] Record { metadata: Metadata { level: Debug, target: "russh::ssh_read" }, args: "U1NILTIuMC1PcGVuU1NIXzkuNnAxIERlYmlhbi00DQo", module_path: Some(Static("russh::ssh_read")), file: Some(Static("/home/honey/workspace/rehino/russh/src/ssh_read.rs")), line: Some(133) }
[2024-07-09T02:24:19Z DEBUG russh::server::kex] Record { metadata: Metadata { level: Debug, target: "russh::server::kex" }, args: server kex init: [20, 166, 99, 205, 72, 200, 57, 215, 20, 204, 242, 222, 122, 122, 13, 6, 242, 0, 0, 0, 146, 99, 117, 114, 118, 101, 50, 53, 53, 49, 57, 45, 115, 104, 97, 50, 53, 54, 44, 99, 117, 114, 118, 101, 50, 53, 53, 49, 57, 45, 115, 104, 97, 50, 53, 54, 64, 108, 105, 98, 115, 115, 104, 46, 111, 114, 103, 44, 100, 105, 102, 102, 105, 101, 45, 104, 101, 108, 108, 109, 97, 110, 45, 103, 114, 111, 117, 112, 49, 54, 45, 115, 104, 97, 53, 49, 50, 44, 100, 105, 102, 102, 105, 101, 45, 104, 101, 108, 108, 109, 97, 110, 45, 103, 114, 111, 117, 112, 49, 52, 45, 115, 104, 97, 50, 53, 54, 44, 101, 120, 116, 45, 105, 110, 102, 111, 45, 115, 44, 107, 101, 120, 45, 115, 116, 114, 105, 99, 116, 45, 115, 45, 118, 48, 48, 64, 111, 112, 101, 110, 115, 115, 104, 46, 99, 111, 109, 0, 0, 0, 24, 115, 115, 104, 45, 101, 100, 50, 53, 53, 49, 57, 44, 114, 115, 97, 45, 115, 104, 97, 50, 45, 50, 53, 54, 0, 0, 0, 85, 99, 104, 97, 99, 104, 97, 50, 48, 45, 112, 111, 108, 121, 49, 51, 48, 53, 64, 111, 112, 101, 110, 115, 115, 104, 46, 99, 111, 109, 44, 97, 101, 115, 50, 53, 54, 45, 103, 99, 109, 64, 111, 112, 101, 110, 115, 115, 104, 46, 99, 111, 109, 44, 97, 101, 115, 50, 53, 54, 45, 99, 116, 114, 44, 97, 101, 115, 49, 57, 50, 45, 99, 116, 114, 44, 97, 101, 115, 49, 50, 56, 45, 99, 116, 114, 0, 0, 0, 85, 99, 104, 97, 99, 104, 97, 50, 48, 45, 112, 111, 108, 121, 49, 51, 48, 53, 64, 111, 112, 101, 110, 115, 115, 104, 46, 99, 111, 109, 44, 97, 101, 115, 50, 53, 54, 45, 103, 99, 109, 64, 111, 112, 101, 110, 115, 115, 104, 46, 99, 111, 109, 44, 97, 101, 115, 50, 53, 54, 45, 99, 116, 114, 44, 97, 101, 115, 49, 57, 50, 45, 99, 116, 114, 44, 97, 101, 115, 49, 50, 56, 45, 99, 116, 114, 0, 0, 0, 123, 104, 109, 97, 99, 45, 115, 104, 97, 50, 45, 53, 49, 50, 45, 101, 116, 109, 64, 111, 112, 101, 110, 115, 115, 104, 46, 99, 111, 109, 44, 104, 109, 97, 99, 45, 115, 104, 97, 50, 45, 50, 53, 54, 45, 101, 116, 109, 64, 111, 112, 101, 110, 115, 115, 104, 46,99, 111, 109, 44, 104, 109, 97, 99, 45, 115, 104, 97, 50, 45, 53, 49, 50, 44, 104, 109, 97, 99, 45, 115, 104, 97, 50, 45, 50, 53, 54, 44, 104, 109, 97, 99, 45, 115, 104, 97, 49, 45, 101, 116, 109, 64, 111, 112, 101, 110, 115, 115, 104, 46, 99, 111, 109, 44, 104, 109, 97, 99, 45, 115, 104, 97, 49, 0, 0, 0, 123, 104, 109, 97, 99, 45, 115, 104, 97, 50, 45, 53, 49, 50, 45, 101, 116, 109, 64, 111, 112, 101, 110, 115, 115, 104, 46, 99, 111, 109, 44, 104, 109, 97, 99, 45, 115, 104, 97, 50, 45, 50, 53, 54, 45, 101, 116, 109, 64, 111, 112, 101, 110, 115, 115, 104, 46, 99, 111, 109, 44, 104, 109, 97, 99, 45, 115, 104, 97, 50, 45, 53, 49, 50, 44, 104, 109, 97, 99, 45, 115, 104, 97, 50, 45, 50, 53, 54, 44, 104, 109, 97, 99, 45, 115, 104, 97, 49, 45, 101, 116, 109, 64, 111, 112, 101, 110, 115, 115, 104, 46, 99, 111, 109, 44, 104, 109, 97, 99, 45, 115, 104, 97, 49, 0, 0, 0, 26, 110, 111, 110, 101, 44, 122, 108, 105, 98, 44, 122, 108, 105, 98, 64, 111, 112, 101, 110, 115, 115, 104, 46, 99, 111, 109, 0, 0, 0, 26, 110, 111, 110, 101, 44, 122, 108, 105, 98, 44, 122, 108, 105, 98, 64, 111, 112, 101, 110, 115, 115, 104, 46, 99, 111, 109, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], module_path: Some(Static("russh::server::kex")), file: Some(Static("russh/src/server/kex.rs")), line: Some(69) }
[2024-07-09T02:24:19Z DEBUG russh::cipher] Record { metadata: Metadata { level: Debug, target: "russh::cipher" }, args: writing, seqn = 0, module_path: Some(Static("russh::cipher")), file: Some(Static("/home/honey/workspace/rehino/russh/src/cipher/mod.rs")), line: Some(166) }
[2024-07-09T02:24:19Z DEBUG russh::cipher] Record { metadata: Metadata { level: Debug, target: "russh::cipher" }, args: padding length 7, module_path: Some(Static("russh::cipher")), file: Some(Static("/home/honey/workspace/rehino/russh/src/cipher/mod.rs")), line: Some(169) }
[2024-07-09T02:24:19Z DEBUG russh::cipher] Record { metadata: Metadata { level: Debug, target: "russh::cipher" }, args: packet_length 708, module_path: Some(Static("russh::cipher")), file: Some(Static("/home/honey/workspace/rehino/russh/src/cipher/mod.rs")), line: Some(171) }
[2024-07-09T02:24:19Z DEBUG russh::ssh_read] Record { metadata: Metadata { level: Debug, target: "russh::ssh_read" }, args: id 32 32, module_path: Some(Static("russh::ssh_read")), file: Some(Static("/home/honey/workspace/rehino/russh/src/ssh_read.rs")), line: Some(66) }
[2024-07-09T02:24:19Z DEBUG russh::cipher] Record { metadata: Metadata { level: Debug, target: "russh::cipher" }, args: reading, len = [0, 0, 5, 252], module_path: Some(Static("russh::cipher")), file: Some(Static("/home/honey/workspace/rehino/russh/src/cipher/mod.rs")), line: Some(207) }
```

Each client (source) gets a session ID. The session ID is made from two elements, a client counter, an underscore, then a UUIDv4. The example log data shows that this is the third client to connect to the honeypot, and it has been assigned session ID of "3_a38347f1-56e3-4bde-83d5-4eed5da57489". We can also tell this client sent in a debian openssh 9.6 version string. The base64 encoding of the debug data doesn't use padding.

#### TODO

- expand logging of russh to leverage session id as well
- fix/expand logging regarding RSA exponents in public keys (currently each client public key exponent is debug logged as the byte array of [1, 0, 1] haha)
- support additional kex algorith modules (and add some additional kex algos in said detached modules)






