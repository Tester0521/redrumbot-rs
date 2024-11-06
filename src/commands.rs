use teloxide::utils::command::BotCommands;
use teloxide::prelude::*;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Команды:")]
pub enum Command {
    #[command(description = "Запустить бота")]
    Start,
    #[command(description = "Выбрать модель")]
    Select,
    #[command(description = "Генерация qr", parse_with = "split")]
    Qr { data: String, version: i16, style: String },
    #[command(parse_with = "split")]
    Whisper { to: i64, msg: String },
    #[command(description = "Помощь")]
    Help,
}

#[derive(BotCommands, PartialEq, Debug)]
#[command(rename_rule = "lowercase")]
pub enum AdminCommand {
    #[command(parse_with = "split")]
    Whisper { to: i64, msg: String },
}


// fn parse_whisper(input: String) -> Result<Command, dyn Error> {
//     let mut parts = input.splitn(2, ' '); 

//     let to = parts
//         .next()
//         .ok_or("Не удалось получить параметр 'to'")?
//         .parse::<i64>()?;
    
//     let msg = parts
//         .next()
//         .ok_or("Не удалось получить параметр 'msg'")?
//         .to_string();
    
//     Ok(Command::Whisper {to, msg})
// }
