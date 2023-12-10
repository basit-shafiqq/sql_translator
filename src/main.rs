use reqwest;
use serde_derive::{Deserialize, Serialize};
use spinners::{Spinner, Spinners};
use std::env;
use std::io::{self, Write};

#[derive(Deserialize, Debug)]
struct OAIChoices {
    text: String,
    index: u8,
    logprobs: Option<u8>,
    finish_reason: String,
}

#[derive(Deserialize, Debug)]
struct OAIResponse {
    id: Option<String>,
    object: Option<String>,
    created: Option<u64>,
    model: Option<String>,
    choices: Vec<OAIChoices>,
}

#[derive(Serialize, Debug)]
struct OAIRequest {
    prompt: String,
    max_tokens: u16,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    dotenv::dotenv().ok();

    let uri = "https://api.openai.com/v1/engines/text-davinci-001/completions";
    let preamble = "Write a sql query for";
    let oai_token: String = env::var("OAI_TOKEN").unwrap();
    let auth_header_val = format!("Bearer {}", oai_token);

    println!("{esc}c", esc = 27 as char);

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut user_text = String::new();

        println!("Enter you query:");
        io::stdin().read_line(&mut user_text).expect("Failed to read line");
        println!();

        let sp = Spinner::new(&Spinners::Dots12, "\t\tOpenAI is Thinking...".into());

        let oai_request = OAIRequest {
            prompt: format!("{} {}", preamble, user_text),
            max_tokens: 10,
        };

        let client = reqwest::Client::new();
        let res = client
            .post(uri)
            .header("Content-Type", "application/json")
            .header("Authorization", &auth_header_val)
            .json(&oai_request)
            .send()
            .await?;

        let json: OAIResponse = res.json().await?;

        sp.stop();
        println!();
        println!("{}", json.choices[0].text);
    }

    Ok(())
}
