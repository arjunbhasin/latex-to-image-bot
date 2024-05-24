// This module contains the implementation of the LaTeX backend for the
// bot. It uses the `latex_to_image.py` script to convert LaTeX expressions
// to images. The `convert_expression_to_image` function reads the incoming
// message, takes the first 200 characters, and executes the Python script
// with the LaTeX expression as an argument. The output image is then sent
// back to the user.

use std::process;
use std::path::Path;
use teloxide::prelude::*;
use teloxide::types::InputFile;
use teloxide::adaptors::Throttle;

pub async fn convert_expression_to_image(bot: Throttle<Bot>, msg: Message) -> ResponseResult<()> {
    // check if the incoming message is a text message of length < 200 characters
    if msg.text().unwrap_or_default().len() > 200 {
        bot.send_message(msg.chat.id, "Expression must be less than 200 characters.")
            .await
            .log_on_error()
            .await;
        return respond(());
    }

    // Send a message to the user indicating that the image is being prepared
    let _ = bot.send_message(msg.chat.id, "Please wait while I prepare the image for you...").await;
        
    // Read the incoming message and take the first 100 characters
    let latex_expr: String = msg.text().unwrap_or_default().chars().take(100).collect();

    // Define the output image file name
    // The file name is based on the chat ID and the string "latex_output.png"
    let output_image: &str = &format!("{}_latex_output.png", msg.chat.id);

    // Execute the Python script with `latex_expr` as an argument
    let status = process::Command::new("python")
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
}