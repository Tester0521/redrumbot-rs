use teloxide::prelude::*;
use teloxide::types::{User, InlineKeyboardMarkup, InlineKeyboardButton, KeyboardButton, ParseMode};
use teloxide::utils::command::BotCommands;
use reqwest::Client as HttpClient;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::Mutex;


#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Команды:")]
enum Command {
    #[command(description = "Запустить бота")]
    Start,
    #[command(description = "Выбрать модель")]
    Select,
    #[command(description = "Помощь")]
    Help,
}


async fn gen_res(prompt: &str, selected_model: Arc<Mutex<String>>) -> Result<String, reqwest::Error> {
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

async fn invoke(bot: Bot, message: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Select => {
            let keyboard = InlineKeyboardMarkup::new(
                vec![
                    vec![InlineKeyboardButton::callback("llama3.1:8b", "llama3.1:8b")],
                    vec![InlineKeyboardButton::callback("llama3.1:70b", "llama3.1:70b")],
                ]);
            bot.send_message(message.chat.id, "Выберите модель:")
                .reply_markup(keyboard)
                .await?
        }
        Command::Help => bot.send_message(message.chat.id, Command::descriptions().to_string()).await?,
        Command::Start => bot.send_message(message.chat.id,"Спрашивай что угодно! Я умнее ЧатГПТ").await?,

    };

    Ok(())
}

async fn callback_handler(bot: Bot, query: CallbackQuery, selected_model: Arc<Mutex<String>>) -> ResponseResult<()> {
    if let Some(data) = query.data {
        let mut selected_model_lock = selected_model.lock().await;
        *selected_model_lock = data.clone();

        if let Some(message) = query.message {
            bot.send_message(message.chat().id, format!("Вы выбрали модель: {}", data))
                .await?;
        }
    }

    Ok(())
}


#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting bot...");
    println!("Starting bot...");

    let bot = Bot::from_env();
    let selected_model = Arc::new(Mutex::new(String::from("llama3.1:8b")));


    let message_handler = dptree::entry().branch(Update::filter_message().endpoint({
        let selected_model = Arc::clone(&selected_model);

        move |bot: Bot, message: Message| {
            let bot = bot.clone();
            let selected_model = Arc::clone(&selected_model);

            async move {
                if let Some(text) = message.text() {
                    if let Ok(cmd) = Command::parse(text, "RedrumAI_bot") {
                        invoke(bot, message, cmd).await?;
                    } else {
                        let prompt = text.to_string();
                        let processing_msg = bot.send_message(message.chat.id, "В обработке...").await?;
                        let user: &User = message.from().expect("User is not found");

                        match &user.username {
                            Some(username) => println!("\n\n################################################\nЗапрос {} в обработке...\n{}", username, prompt),
                            None => println!("\n\n################################################\nЗапрос Аноним в обработке...\n{}", prompt),
                        };
                            
                        if let Ok(res) = gen_res(&prompt, selected_model).await {
                            bot.delete_message(message.chat.id, processing_msg.id).await?;
                            bot.send_message(message.chat.id, res).parse_mode(ParseMode::Markdown).await?;
                        } else {
                            bot.delete_message(message.chat.id, processing_msg.id).await?;
                            bot.send_message(message.chat.id, "Сорри, я сломался :(( Попробуйте повторить запрос.").await?;
                        }
                    }
                }
                Ok::<_, teloxide::RequestError>(())
            }
        }
    })).branch(Update::filter_callback_query().endpoint({
        let bot = bot.clone();
        let selected_model = Arc::clone(&selected_model);
        move |query: CallbackQuery| {
            println!("callback");
            let bot = bot.clone();
            let selected_model = Arc::clone(&selected_model);
            async move {
                callback_handler(bot, query, selected_model).await?;
                Ok::<_, teloxide::RequestError>(())
            }
        }
    }));

    Dispatcher::builder(bot.clone(), message_handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

