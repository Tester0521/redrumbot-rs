
use reqwest::Client as HttpClient;
use serde_json::Value;
use std::sync::Arc;
use std::io::Cursor;
use std::error::Error;
use tokio::sync::Mutex;
use image::{DynamicImage, ImageFormat};
use image::imageops::FilterType;

use crate::qr::{ QrCat, QrStyle };

pub async fn gen_res(prompt: &str, selected_model: Arc<Mutex<String>>) -> Result<String, reqwest::Error> {
    let model = selected_model.lock().await.to_owned(); 
    let prompt_formatted = prompt.replace("\\", "\\\\").replace("{}", "{{}}").replace("\"", "\\\"").replace("\n", " ");
    let data = format!(r#"
        {{
            "model": "{}",
            "prompt": "{}",
            "stream": false
        }}
    "#, model, prompt_formatted);
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

pub async fn gen_qr(data: &str, version: i16, style: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let style = match style {
        "default" => QrStyle::Default,
        "half" => QrStyle::Half,
        "rounded" => QrStyle::Rounded,
        _ => QrStyle::Default,
    };
    let mut buffer = Vec::new();
    let mut cursor = Cursor::new(&mut buffer);
    let image = QrCat::new().version(version).data(&data).style(style).build();
    let dynamic_image = DynamicImage::ImageRgba8(image?).resize(1024, 1024, FilterType::Triangle);

    dynamic_image.write_to(&mut cursor, ImageFormat::Png)?;

    Ok(buffer)
}
