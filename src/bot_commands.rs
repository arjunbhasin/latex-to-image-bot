use teloxide::{
    prelude::*,
    utils::command::BotCommands
};
use teloxide::adaptors::Throttle;
use crate::keyboard::construct_main_menu_keyboard;
use crate::gorq::get_response_from_groq;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum Command {
    #[command(description = "start the bot.")]
    Start,
    #[command(description = "show help.")]
    Help,
    #[command(description = "show categories for common expressions.")]
    Category,
    #[command(description = "show latex syntax of")]
    Syntax(String),
}

pub async fn handle_command(bot: Throttle<Bot>, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {

        Command::Start => {
            let welcome_message = "Welcome to the @latex2image Bot!\n
Here are the features you can use:\n
1. <b>Convert LaTeX to Images</b>: Easily convert your LaTeX expressions into images for easy sharing and usage.\n\nType $e^{ix} + 1 = 0$ to get started.\n 
2. <b>Generate LaTeX Expressions from Text</b>: Use advanced LLMs to convert natural language descriptions into LaTeX expressions. \n\nTry `/syntax Binomial Theorem`.\n
3. <b>Pre-built Expressions</b>: Access a library of common math and physics LaTeX expressions.\n\nRun the  `/category` command to see the list.\n

If you need more help use the `/help` command.
";

            bot.send_message(
                msg.chat.id, 
                welcome_message
            )
            .parse_mode(teloxide::types::ParseMode::Html)
            .await?;
        }
        Command::Help => {
            let help_message = "Here is a list of the supported commands
            \n/start - Start the bot.
            \n/help - Show help.
            \n/category - Show categories for common expressions.
            \n/syntax 'some text' - Show latex syntax of a given text.
            \n\nTo get the image just type the latex expression in the chat.
            \nExamples:
            \n$e^{ix} + 1 = 0$
            \n$\\frac{1}{2}$
            \n$\\int_{0}^{\\infty} e^{-x^2} dx$
            ";
            bot.send_message(
                msg.chat.id,
                help_message,
            )
            .parse_mode(teloxide::types::ParseMode::Html)
            .await?;
        }
        Command::Category => {

            let keyboard = construct_main_menu_keyboard().await;

            bot.send_message(msg.chat.id, "Choose a Category:")
                .reply_markup(keyboard)
                .await?;
        }
        Command::Syntax(prompt) => {
            // first check if the prompt is <= 30 characters
            if prompt.len() > 50 {
                bot.send_message(msg.chat.id, "The prompt must be less than 50 characters.").await?;
            }
            else {
                match get_response_from_groq(prompt).await {
                    Ok(response) => {
                        bot.send_message(msg.chat.id, response).await?;
                    },
                    Err(_) => {
                        bot.send_message(msg.chat.id, "Oops! Failed to get a response from LLM\nPlease try after sometime.").await?;
                    }
                }
            }
        }
    }
    Ok(())
}
