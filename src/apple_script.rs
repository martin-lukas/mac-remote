use std::process::Command;

pub fn trigger_keys(keys: Vec<&str>) {
    for key in keys {
        match key {
            "F" => execute("tell application \"System Events\" to keystroke \"f\""),
            "SPACE" => execute("tell application \"System Events\" to keystroke space"),
            "LEFT" => execute("tell application \"System Events\" to key code 123"),
            "RIGHT" => execute("tell application \"System Events\" to key code 124"),
            "DOWN" => execute("tell application \"System Events\" to key code 125"),
            "UP" => execute("tell application \"System Events\" to key code 126"),
            _ => panic!("Unsupported keys were triggered: {}", key),
        }
    }
}

fn execute(script: &str) {
    println!("{:?}", script);
    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .expect("Failed to execute command");
    println!("{:?}", output);
}
