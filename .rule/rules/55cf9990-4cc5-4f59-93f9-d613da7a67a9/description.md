The binary is `~/.cargo/bin/ticket-viewer.exe`. Building only `-p ticket-http` produces
the library but not the binary; the server will be stale until the viewer crate is rebuilt.