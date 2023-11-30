# MacRemote - control your Mac from your phone

- Simple CLI app for controlling your Mac with your phone.
- Ideal for connecting your laptop to your TV, and then opening the URL on your phone to send commands to it.
- Consists of a web server running on your laptop, which will make a Web UI accessible on the laptop's local IP address, to be able to access it e.g. on your phone connected to the same network.
- Written in Rust for performance and low RAM usage (just a few MBs).

## Run locally

To run the server:

1. Build the server: `cargo build --release`
2. Run the binary: `./target/release/macremote`
3. You can also direct the logs to a file, by adding `> debug.log` after it.
4. The server should now be running on port `8000`, with the UI accessible on:
    - [http://127.0.0.1:8000](http://127.0.0.1:8000)
    - [http://0.0.0.0:8000](http://0.0.0.0:8000)
    - The laptop's local IP address (in MacOS, run `ifconfig 'en0' | grep 'inet '`) + the port.
5. If you want to make this IP stable, you should configure your router to assign
   a stable IP address to that laptop. Then you can bookmark it on your phone for easy access.

## Create a launch script

You can create a simple bash script for launching the server from anywhere in terminal

1. Create a new file with:
    ```sh
    #!/bin/bash
    ABSOLUTE_PATH_TO_PROJECT/target/release/macremote > ABSOLUTE_PATH_TO_LOG_DIR/debug.log
    ```
5. Make the script executable: `chmod +x macremote`
6. The folder the script is in should be added to the `PATH` env. variable for easy execution from anywhere in a terminal, with just `macremote`


