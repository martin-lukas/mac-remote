# MacRemote - control your Mac from your phone (Rust version)

To run the server:

1. Build the server: `cargo build --release`
2. Run the binary: `./target/release/mac-remote`
3. You can also direct the logs to a file, by adding `> debug.log` after it.
4. Optionally, you can create a simple bash script for launching the server from anywhere in terminal, e.g.:
    ```sh
    #!/bin/bash

    ABSOLUTE_PATH_TO_PROJECT/target/release/mac-remote > ABSOLUTE_PATH_TO_LOG_DIR/debug.log
    ```
5. Make the script executable, e.g. `chmod +x macremote`
6. Add the directory the script above is in to `PATH` env. variable.

The server itself should start on port 8050, and be accessible also via the PC's IP address (not just localhost) on the lcoal network.
