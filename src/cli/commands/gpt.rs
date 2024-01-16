// cli/commands/gpt.rs

use ez::features::gpt::gpt_client::{GptClient,GptError};
use crate::cli::commands::command_trait::Command;
use crate::cli::colors;
use async_trait::async_trait;

pub struct GptCommand;


#[async_trait]
impl Command for GptCommand {
    async fn run(&self, args: &str) {
        let mut gpt_client = GptClient::new();
        gpt_client.set_api_key("api here"); // Replace with your actual API key
        //println!("{}{}USER{}: {}\n", colors::UNDERLINE, colors::MAGENTA, colors::RESET, &args);
        match gpt_client.make_request(args).await {
            Ok(response) => println!("{}{}GPT4{}: {}{}{}", colors::UNDERLINE, colors::WHITE,colors::RESET, colors::YELLOW, response, colors::RESET),
            Err(e) => eprintln!("{} {}",colors::red("ERROR: "), e), // General error handling
        }
        
    }

    async fn helper(&self) {
        println!("Gpt Command: -gpt prompt");
    }
}
