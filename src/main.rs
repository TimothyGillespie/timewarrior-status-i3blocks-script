use serde_json::json;

fn main() {
    let result = json!({
       "short_text": "test text",
        "full_text": "full_text replaced"
    });

    println!("{}", result.to_string());
}
