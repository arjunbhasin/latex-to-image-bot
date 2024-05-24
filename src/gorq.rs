use reqwest::Client;
use serde::Serialize;
use serde::Deserialize;
use std::env;
use std::time::Duration;

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct RequestBody {
    messages: Vec<Message>,
    model: String,
    temperature: f64,
    max_tokens: usize,
    top_p: f64,
    stream: bool,
    stop: Option<String>,
}

#[derive(Deserialize)]
struct Choice {
    message: AssistantMessage,
}

#[derive(Deserialize)]
struct AssistantMessage {
    content: String,
}

#[derive(Deserialize)]
struct ResponseBody {
    choices: Vec<Choice>,
}

pub async fn get_response_from_groq(prompt: String) -> Result<String, reqwest::Error> {
    let api_key = env::var("GROQ_API_KEY").expect("GROQ_API_KEY must be set");

    let client = Client::builder()
    .timeout(Duration::from_secs(10))
    .build()?;

    let request_body = RequestBody {
        messages: vec![
            Message {
                role: "user".to_string(),
                content: "how do we write '".to_owned() + &prompt + "' in latex. Give the expression only in format $...$",
            }
        ],
        model: "llama3-8b-8192".to_string(),
        temperature: 1.0,
        max_tokens: 1024,
        top_p: 1.0,
        stream: false,
        stop: None,
    };

    let response = client
        .post("https://api.groq.com/openai/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request_body)
        .send()
        .await;
    
    match response {
        Ok(resp) => {
            let response_body: ResponseBody = resp.json().await?;
            if let Some(choice) = response_body.choices.first() {
                Ok(choice.message.content.clone())
            } else {
                Ok("Oops! No response from LLM.".to_string())
            }
        }
        Err(err) => {
            eprintln!("Request error: {:?}", err);  // Log the error
            Err(err)
        }
    }

}