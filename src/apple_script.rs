use std::process::{Command, Output};

pub fn trigger_keys(keys: Vec<&str>) {
    println!("Triggering keys: {}", keys.join(", "));
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
    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .expect(format!("Failed to execute command '{}'", script).as_str());
    pretty_print_output(output);
}

fn pretty_print_output(output: Output) {
    let result = match output.status.success() {
        true => "OK".to_string(),
        false => match String::from_utf8(output.stderr) {
            Ok(err_str) => err_str,
            Err(err) => format!("Couldn't interpret string from bytes: {:?}", err),
        },
    };
    println!("Result: {result}");
}
