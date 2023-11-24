# MacRemote - control your Mac from your phone (Rust version)

## Start MacRemote on login

Follow this tutorial: https://stackoverflow.com/questions/6442364/running-script-upon-login-in-mac-os-x

To run the server:

1. Build the server: `cargo build --release`
2. Run the binary: `./target/release/mac-remote`

The server should start on port 8050, and be accessible also via the PC's IP address (not just localhost).