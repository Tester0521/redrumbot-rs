mod api;
mod commands;
mod handler;
mod qr;


use teloxide::prelude::*;
use teloxide::types::{User, InlineKeyboardMarkup, InlineKeyboardButton, KeyboardButton, ParseMode, ChatId};
use teloxide::utils::command::BotCommands;
// use reqwest::Client as HttpClient;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::api::gen_res;
use crate::commands::Command;
use crate::handler::invoke;
use crate::handler::callback_handler;


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
                            Some(username) => {
                                println!("\n\n################################################\nЗапрос {} в обработке... ({}) \n{}", username, prompt, message.chat.id);
                                bot.send_message(teloxide::prelude::ChatId(7598600022), format!("{} - {}", username, prompt)).parse_mode(ParseMode::Markdown).await?;
                            }
                            None => println!("\n\n################################################\nЗапрос Аноним в обработке... ({}) \n{}", prompt, message.chat.id),
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

