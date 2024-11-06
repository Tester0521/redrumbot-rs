mod api;
mod commands;
mod handler;
mod qr;
mod users;


use teloxide::prelude::*;
use teloxide::types::{User, InlineKeyboardMarkup, InlineKeyboardButton, KeyboardButton, ParseMode, ChatId};
use teloxide::utils::command::BotCommands;
// use reqwest::Client as HttpClient;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::api::gen_res;
use crate::commands::Command;
use crate::commands::AdminCommand;
use crate::handler::invoke;
use crate::handler::admin_invoke;
use crate::handler::callback_handler;
use crate::users::load_users;
use crate::users::save_user;
use crate::users::UserData;





async fn handle_command(bot: Bot, message: Message, cmd: Command, users_data: Arc<Mutex<Vec<UserData>>>, selected_model: Arc<Mutex<String>>) -> Result<(), teloxide::RequestError> {
    invoke(bot, message, cmd).await?;
    Ok(())
}

async fn handle_admin_command(bot: Bot, message: Message, cmd: AdminCommand) -> Result<(), teloxide::RequestError> {
    admin_invoke(bot, message, cmd).await?;
    Ok(())
}

async fn handle_message(bot: Bot, message: Message, users_data: Arc<Mutex<Vec<UserData>>>, selected_model: Arc<Mutex<String>>) -> Result<(), teloxide::RequestError> {
    if let Some(text) = message.text() {
        let prompt = text.to_string();
        let processing_msg = bot.send_message(message.chat.id, "В обработке...").await?;
        let user: &User = message.from().expect("User is not found");

        match &user.username {
            Some(username) => {
                println!("\n\n################################################\nЗапрос {} в обработке... ({}) \n{}", username, prompt, message.chat.id);
                bot.send_message(teloxide::prelude::ChatId(7598600022), format!("{} - {} ({})", username, prompt, message.chat.id.0)).parse_mode(ParseMode::Markdown).await?;
                save_user("data.json", &users_data, message.chat.id.0, username).await.unwrap();
            }
            None => println!("\n\n################################################\nЗапрос Аноним в обработке... ({}) \n{}", prompt, message.chat.id),
        }
                                
        if let Ok(res) = gen_res(&prompt, selected_model).await {
            bot.delete_message(message.chat.id, processing_msg.id).await?;
            bot.send_message(message.chat.id, res).parse_mode(ParseMode::Markdown).await?;
        } else {
            bot.delete_message(message.chat.id, processing_msg.id).await?;
            bot.send_message(message.chat.id, "Сорри, я сломался :(( Попробуйте повторить запрос.").await?;
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
    let users_data = Arc::new(Mutex::new(load_users("data.json").await.unwrap()));


    let message_handler = dptree::entry()
        .branch(Update::filter_message().filter_command::<Command>().endpoint({
            let selected_model = Arc::clone(&selected_model);
            let users_data = Arc::clone(&users_data);
                
            move |bot: Bot, message: Message, cmd: Command| {
                let users_data = Arc::clone(&users_data);
                let selected_model = Arc::clone(&selected_model);

                async move {
                    handle_command(bot, message, cmd, users_data, selected_model).await
                }
            }
        }))
        // .branch(Update::filter_message().filter_command::<AdminCommand>().filter(|msg: &Message| msg.from().map_or(false, |user| 7598600022 == user.id.0)).endpoint({
        //     move |bot: Bot, message: Message, cmd: AdminCommand| {
        //         async move {
        //             handle_admin_command(bot, message, cmd).await
        //         }
        //     }
        // }))
        .branch(Update::filter_message().filter(|msg: Message| Command::parse(msg.text().unwrap_or(""), "RedrumAI_bot").is_err()).endpoint({
            let selected_model = Arc::clone(&selected_model);
            let users_data = Arc::clone(&users_data);

            move |bot: Bot, message: Message| {
                let users_data = Arc::clone(&users_data);
                let selected_model = Arc::clone(&selected_model);

                async move {
                    handle_message(bot, message, users_data, selected_model).await
                }
            }
        }))
        .branch(Update::filter_callback_query().endpoint({
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




// .branch(Update::filter_message().endpoint({
//         let selected_model = Arc::clone(&selected_model);
//         let users_data = Arc::clone(&users_data);

//         move |bot: Bot, message: Message| {
//             let bot = bot.clone();
//             let selected_model = Arc::clone(&selected_model);
//             let users_data = Arc::clone(&users_data);

//             async move {
//                 if let Some(text) = message.text() {
//                     if let Ok(cmd) = Command::parse(text, "RedrumAI_bot") {
//                         invoke(bot, message, cmd).await?;
//                     } else {
//                         // let prompt = text.to_string();
//                         // let processing_msg = bot.send_message(message.chat.id, "В обработке...").await?;
//                         // let user: &User = message.from().expect("User is not found");

//                         // match &user.username {
//                         //     Some(username) => {
//                         //         println!("\n\n################################################\nЗапрос {} в обработке... ({}) \n{}", username, prompt, message.chat.id);
//                         //         bot.send_message(teloxide::prelude::ChatId(7598600022), format!("{} - {} ({})", username, prompt, message.chat.id.0)).parse_mode(ParseMode::Markdown).await?;
//                         //         save_user("data.json", &users_data, message.chat.id.0, username).await.unwrap();
//                         //     }
//                         //     None => println!("\n\n################################################\nЗапрос Аноним в обработке... ({}) \n{}", prompt, message.chat.id),
//                         // }
                            
//                         // if let Ok(res) = gen_res(&prompt, selected_model).await {
//                         //     bot.delete_message(message.chat.id, processing_msg.id).await?;
//                         //     bot.send_message(message.chat.id, res).parse_mode(ParseMode::Markdown).await?;
//                         // } else {
//                         //     bot.delete_message(message.chat.id, processing_msg.id).await?;
//                         //     bot.send_message(message.chat.id, "Сорри, я сломался :(( Попробуйте повторить запрос.").await?;
//                         // }

//                         handle_message(bot, message, users_data, selected_model).await;
//                     }
//                 }
//                 Ok::<_, teloxide::RequestError>(())
//             }
//         }
//     }))