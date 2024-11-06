use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardMarkup, InlineKeyboardButton, CallbackQuery};
use teloxide::types::InputFile;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::commands::Command;
use crate::commands::AdminCommand;
use crate::api::gen_qr;

pub async fn invoke(bot: Bot, message: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Select => {
            let keyboard = InlineKeyboardMarkup::new(
                vec![
                    vec![InlineKeyboardButton::callback("v1 default", "llama3.1:8b")],
                    // vec![InlineKeyboardButton::callback("v2 (32gb ram)", "llama3.1:70b")],
                ]);
            bot.send_message(message.chat.id, "Выберите модель:")
                .reply_markup(keyboard)
                .await?
        },
        Command::Qr { data, version, style } => {
            println!("Генерация... ({})", message.chat.id);

            let buffer = gen_qr(&data, version, &style).await.unwrap();
            let photo = InputFile::memory(buffer);
            bot.send_photo(message.chat.id, photo).await?
        },
        Command::Whisper { to, msg } => {
            if let 7598600022 = message.chat.id.0 {
                bot.send_message(teloxide::prelude::ChatId(to), msg.clone()).await?;
            }

            bot.send_message(teloxide::prelude::ChatId(7598600022), format!("{} - {} - {}", message.chat.id.0, to, msg)).await?
        },
        Command::Help => bot.send_message(message.chat.id, "Никто тебе не поможет...").await?,
        Command::Start => bot.send_message(message.chat.id, "Спрашивай что угодно! Я умнее ЧатГПТ").await?,
    };

    Ok(())
}

pub async fn admin_invoke(bot: Bot, message: Message, cmd: AdminCommand) -> ResponseResult<()> {
    if let 7598600022 = message.chat.id.0 {
        match cmd {
            AdminCommand::Whisper { to, msg } => bot.send_message(teloxide::prelude::ChatId(to), msg.clone()).await?,
        };
    }


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
