use teloxide::prelude::*;
use std::process::Command;
use std::path::Path;
use teloxide::types::InputFile;

#[tokio::main]
async fn main() {
    let bot = Bot::from_env();

    teloxide::repl(bot, |bot:Bot, msg: Message| async move {
        // Read the incoming message and take the first 25 characters
        let latex_expr: String = msg.text().unwrap_or_default().chars().take(25).collect();

        // Define the output image file name
        let output_image = "latex_output.png";

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

        // Optionally, delete the image file after sending it
        std::fs::remove_file(output_image).ok();

        respond(())
    }).await;
}
