# MacRemoteRust - control your Mac from your phone (Rust version)

## Start MacRemote on login

Follow this tutorial: https://stackoverflow.com/questions/6442364/running-script-upon-login-in-mac-os-x

To run the server, the current directory should be the one with `Rocket.toml` file, and then run the binary `target/release/mac-remote`.

This implementation consumes cca 14 MB of memory, compared the the Scala implementation running on 176 MB.