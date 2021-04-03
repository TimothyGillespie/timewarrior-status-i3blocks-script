use serde_json::json;
use std::process::Command;

fn main() {

    let timewarrior_console_option = Command::new("timew")
        .output()
        .unwrap();

    let timewarrior_console_output = match std::str::from_utf8(timewarrior_console_option.stdout.as_slice()) {
        Ok(v) => v.to_string(),
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    // There might be a better solution to ckeck this
    let time_tracking_active = !timewarrior_console_output.contains("no active time tracking");


    let mut full_text = String::from("");
    if !time_tracking_active {

        full_text.push_str("No time tracking active");

    } else {
        let mut output_lines = timewarrior_console_output.lines();
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
