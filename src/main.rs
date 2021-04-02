use serde_json::json;
use std::process::Command;

fn main() {

    let timewarrior_result = Command::new("timew")
        .output()
        .unwrap();

    let timewarrior_output = match std::str::from_utf8(timewarrior_result.stdout.as_slice()) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    let output = timewarrior_output.to_string();
    let time_tracking_active = !output.contains("no active time tracking");


    let mut full_text = String::from("");
    if(!time_tracking_active) {
        full_text.push_str("No time tracking active");
    } else {
        let mut output_lines = output.lines();
        let output_tags: Vec<&str> = output_lines
            .nth(0)
            .unwrap_or_default()
            .split_whitespace()
            .skip(1)
            .collect();

        let time_tracking_info = output_lines
            .last()
            .unwrap_or_default()
            .split_whitespace()
            .last()
            .unwrap_or_default();

        full_text = output_tags.join(" ");

        full_text.push_str(" ");

        full_text.push_str("[");
        full_text.push_str(time_tracking_info);
        full_text.push_str("]");

    }

    let result = json!({
        "full_text": full_text
    });

    println!("{}", result.to_string());
}
