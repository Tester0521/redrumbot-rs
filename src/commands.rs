use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Команды:")]
pub enum Command {
    #[command(description = "Запустить бота")]
    Start,
    #[command(description = "Выбрать модель")]
    Select,
    #[command(description = "Помощь")]
    Help,
}
