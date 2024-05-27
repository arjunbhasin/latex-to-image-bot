mod bot_commands;
mod latex;
mod inline_buttons;
mod keyboard;
mod gorq;

use teloxide::prelude::*;
use teloxide::adaptors::{Throttle, throttle::Limits};

use teloxide::utils::command::BotCommands;
use teloxide::types::BotCommand;

use bot_commands::{Command, handle_command};
use latex::convert_expression_to_image;
use inline_buttons::handle_callback_query;


async fn message_handler(bot: Throttle<Bot>, msg: Message) -> ResponseResult<()> {
    // Check if the message is a command
    match Command::parse(&msg.text().unwrap_or_default(), "latex_to_image_bot") {
        Ok(command) => {
            handle_command(bot, msg, command).await?;
        }
        Err(_) => {
            // first check if the user entered a string starting with '/' but which is not a command
            if msg.text().unwrap_or_default().starts_with('/') {
                bot.send_message(msg.chat.id, "Invalid command! Please see `/help`").await?;
                return Ok(());
            }
            convert_expression_to_image(bot, msg).await?;
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let bot = Bot::from_env();

    // Define the list of commands
    let commands = vec![
        BotCommand::new("help", "display this text."),
        BotCommand::new("syntax {expression}", "get LaTeX syntax of a given expression."),
    ];

    // Set the commands in the menu
    bot.set_my_commands(commands).await.unwrap();

    let bot: Throttle<Bot> = bot.throttle(
        Limits { 
            messages_per_sec_chat: 1,
            messages_per_min_chat: 10, 
            messages_per_min_channel: 10,
            messages_per_sec_overall: 1,
        });

    let handler = dptree::entry()
        .branch(Update::filter_callback_query().endpoint(handle_callback_query))
        .branch(Update::filter_message().endpoint(message_handler));


    Dispatcher::builder(bot, handler)
        .default_handler(|upd| async move {
            println!("Error! Update not handled: {:?}", upd);
        })
        .build()
        .dispatch()
        .await;
}