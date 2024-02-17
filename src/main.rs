use teloxide::prelude::*;
use teloxide::adaptors::{Throttle, throttle::Limits};
use std::process::Command;
use std::path::Path;
use teloxide::types::InputFile;

#[tokio::main]
async fn main() {
    let bot = Bot::from_env();
    // Throttle the bot to prevent spamming
    let bot: Throttle<Bot> = bot.throttle(
        Limits { 
            messages_per_sec_chat: 1,
            messages_per_min_chat: 10, 
            messages_per_min_channel: 10,
            messages_per_sec_overall: 1,
        });

    teloxide::repl(bot, |bot: Throttle<Bot>, msg: Message| async move {

        // check if the incoming message is a text message of length < 100 characters
        if msg.text().unwrap_or_default().len() > 100 {
            bot.send_message(msg.chat.id, "Expression must be less than 100 characters.")
                .await
                .log_on_error()
                .await;
            return respond(());
        }

        // Read the incoming message and take the first 100 characters
        let latex_expr: String = msg.text().unwrap_or_default().chars().take(100).collect();

        // Define the output image file name
        // The file name is based on the chat ID and the string "latex_output.png"
        let output_image: &str = &format!("{}_latex_output.png", msg.chat.id);

        // Execute the Python script with `latex_expr` as an argument
        let status = Command::new("python")
            .arg("./latex_to_image.py")  // Path to your Python script
            .arg(&latex_expr)
            .arg(&output_image)
            .status();

        match status {
            Ok(status) if status.success() => {
                // Check if the output image file exists
                if Path::new(&output_image).exists() {
                    // Send the generated image back to the user
                    bot.send_photo(msg.chat.id, InputFile::file(output_image))
                        .await
                        .log_on_error()
                        .await;
                } else {
                    bot.send_message(msg.chat.id, "Failed to generate LaTeX image.")
                        .await
                        .log_on_error()
                        .await;
                }
            }
            _ => {
                // Handle the case where the script execution failed or command failed to execute
                bot.send_message(msg.chat.id, "Error executing LaTeX conversion script.")
                    .await
                    .log_on_error()
                    .await;
            }
        }

        // Delete the image file after sending it
        std::fs::remove_file(output_image).ok();

        respond(())
    }).await;
}
