# rehino - a russh honeypot

This fork of russh contains additional modification and tooling for the purpose of honeypot engineering, debugging, and more.

The program `rehino` is an ssh honeypot made from the russh echoserver example. It has been adapted to have verbose logging and data capture.

The russh has been modified to support customizing the ssh header packet that displays the version, as to spoof various versions.

The `rehino` program uses cargo patches to pull from the local version of russh rather than crates.io.
