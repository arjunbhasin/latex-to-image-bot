mod bot_commands;
mod latex;
mod inline_buttons;
mod keyboard;
mod gorq;

use teloxide::prelude::*;
use teloxide::adaptors::{Throttle, throttle::Limits};

use teloxide::utils::command::BotCommands;

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
            convert_expression_to_image(bot, msg).await?;
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let bot = Bot::from_env();
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