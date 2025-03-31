use comrak::{markdown_to_html, Options};
use serde_json::Value;

#[tauri::command]
async fn get(input: String) -> String {
    let client = reqwest::Client::new();
    let v: Value = serde_json::from_str(
        std::fs::read_to_string("input.json")
            .expect("Error while reading")
            .replace(
                r#""Who is the best French painter? Answer in one short sentence.""#,
                format!("\"{}\"", input).as_str(),
            )
            .as_str(),
    )
    .expect("GO TO HELL");

    let res = client
        .post("https://api.mistral.ai/v1/chat/completions")
        .bearer_auth(std::env::var("MISTRAL_API_KEY").expect("Error getting API KEY"))
        .json(&v)
        .send()
        .await
        .expect("msg")
        .text()
        .await
        .expect("msg");

    // Parse the response
    let message: String = match res.lines().find(|x| x.contains("content\"")) {
        Some(line) => {
            let (_, a) = line
                .split_once("content\":\"")
                .expect("Failed to parse content");
            let (b, _) = a.split_once("}").expect("Failed to parse content end");
            b.to_string()
        }
        None => "Sorry, I couldn't generate a response.".to_string(),
    };

    markdown_to_html(message.as_str(), &Options::default())
    //return message;
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
