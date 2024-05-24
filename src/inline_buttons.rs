use teloxide::prelude::*;
use teloxide::types::CallbackQuery;
use teloxide::adaptors::Throttle;
use teloxide::types::InputFile;
use std::path::Path;

use crate::keyboard::construct_arithmetic_keyboard;

pub async fn handle_callback_query(bot: Throttle<Bot>, q: CallbackQuery) -> ResponseResult<()> {    
    if let Some(message) = q.message {
        let chat_id = message.chat.id;
        if let Some(data) = q.data {
            match data.as_str() {
                "Arithmetic" => handle_arithmetic_callback(bot, chat_id).await?,
                "Algebra" => handle_algebra_callback(bot, chat_id).await?,
                "Basic Concepts" => handle_arithmetic_basic_concepts(bot, chat_id).await?,
                _ => handle_unknown_callback(bot, chat_id).await?,
            }
        }
    }
    else {
        eprintln!("Error: Callback query does not contain a message.");
    }
    Ok(())
}

async fn handle_arithmetic_callback(bot: Throttle<Bot>, chat_id: ChatId) -> ResponseResult<()> {
    bot.send_message(chat_id, "Please select from the following")
        .reply_markup(construct_arithmetic_keyboard().await)
                .await?;
    Ok(())
}

async fn handle_algebra_callback(bot: Throttle<Bot>, chat_id: ChatId) -> ResponseResult<()> {
    bot.send_message(chat_id, "You selected Algebra.").await?;
    Ok(())
}

async fn handle_unknown_callback(bot: Throttle<Bot>, chat_id: ChatId) -> ResponseResult<()> {
    bot.send_message(chat_id, "Unknown category selected.").await?;
    Ok(())
}

async fn handle_arithmetic_basic_concepts(bot: Throttle<Bot>, chat_id: ChatId)-> ResponseResult<()> {
    // Path to the PDF file
    let pdf_path = Path::new("public/arithmetic.pdf");

    // Send the PDF file
    bot.send_document(chat_id, InputFile::file(pdf_path)).await?;
    Ok(())
}

// async fn handle_probability_callback(bot: Throttle<Bot>, chat_id: ChatId) -> ResponseResult<()> {
//     bot.send_message(chat_id, "You selected Probability.").await?;
//     Ok(())
// }