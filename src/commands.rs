use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Команды:")]
pub enum Command {
    #[command(description = "Запустить бота")]
    Start,
    #[command(description = "Выбрать модель")]
    Select,
    #[command(description = "Генерация qr", parse_with = "split")]
    Qr { data: String, version: i16, style: String },
    #[command(description = "Генерация qr", parse_with = "split")]
    Whisper { to: i64, msg: String },
    #[command(description = "Помощь")]
    Help,
}
