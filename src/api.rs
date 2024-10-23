use reqwest::Client as HttpClient;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn gen_res(prompt: &str, selected_model: Arc<Mutex<String>>) -> Result<String, reqwest::Error> {
    let model = selected_model.lock().await.to_owned(); 
    let data = format!(r#"
        {{
            "model": "{}",
            "prompt": "{}",
            "stream": false
        }}
    "#, model, prompt);
    let client = HttpClient::new();
    let res = client.post("http://localhost:11434/api/generate")
        .body(data)
        .send()
        .await?;
    let text = res.text().await?;

    for line in text.lines() {
        if let Ok(parsed_json) = serde_json::from_str::<Value>(line) {
            if let Some(response) = parsed_json["model"].as_str() {
                println!("{:?}", response);
            }
            if let Some(response) = parsed_json["created_at"].as_str() {
                println!("{:?}", response);
            }
            if let Some(response) = parsed_json["response"].as_str() {
                println!("{:?}\n################################################\n\n", response);
                return Ok(response.to_string());
            }
        } else {
            eprintln!("АШИПКА json");
        }
    }

    Ok("Сорри, я сломался :(((( Попробуйте повторить запрос.".to_string())
}
