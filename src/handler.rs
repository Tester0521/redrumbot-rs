use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardMarkup, InlineKeyboardButton, CallbackQuery};
use teloxide::utils::command::BotCommands;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::commands::Command;

pub async fn invoke(bot: Bot, message: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Select => {
            let keyboard = InlineKeyboardMarkup::new(
                vec![
                    vec![InlineKeyboardButton::callback("v1 default", "llama3.1:8b")],
                    vec![InlineKeyboardButton::callback("v2 (32gb ram)", "llama3.1:70b")],
                ]);
            bot.send_message(message.chat.id, "Выберите модель:")
                .reply_markup(keyboard)
                .await?
        }
        Command::Help => bot.send_message(message.chat.id, Command::descriptions().to_string()).await?,
        Command::Start => bot.send_message(message.chat.id, "Спрашивай что угодно! Я умнее ЧатГПТ").await?,
    };

    Ok(())
}

pub async fn callback_handler(bot: Bot, query: CallbackQuery, selected_model: Arc<Mutex<String>>) -> ResponseResult<()> {
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
