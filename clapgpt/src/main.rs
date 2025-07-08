mod types;
use crate::types::types::{ MessageResponse, OPenAiRequest};
use termimad::MadSkin;
use std::process;
use std::env;
use clap::{Command, Arg};
use dotenvy::dotenv;
use reqwest::Client;
#[tokio::main]
async  fn main() -> Result <(), Box<dyn std::error::Error>> {
     let skin = MadSkin::default();
    dotenv().ok();
    let api_key = env::var("OPEN_AI_API_KEY")?;
    let open_ai_url = env::var("OPEN_AI_URL")?;
    //open ai model data
    let matches = Command::new("clapgpt")
    .about("An ai command-line tool")
    .version("0.1")
    .author("Yilkash")
    .subcommand(
        Command::new("chat")
        .arg(
        Arg::new("cmd_arg")
        .required(true)
        )
    )
    .get_matches();

    match  matches.subcommand() {
        Some(("chat", arg_matches)) => {
            if let Some(cmd_argument) = arg_matches.get_one::<String>("cmd_arg"){
            let body =    OPenAiRequest{
                model: "gpt-4o-mini".to_string(),
                messages: vec![
                    MessageResponse{
                       role:  "user".to_string(),
                        content: cmd_argument.clone()
                    }
                ]
            };
            let client = Client::new();

            let res = client
            .post(open_ai_url)
            .bearer_auth(api_key)
            .json(&body)
            .send()
            .await?;

            let json_response:  serde_json::Value = res.json().await?;
            let content = json_response["choices"][0]["message"]["content"].as_str().unwrap_or("No response from open AI");
            skin.print_text(content);
            }
        },
        _ => {
            eprintln!("Invalid input ");
            process::exit(1);
        }
    }
    Ok(())
}
