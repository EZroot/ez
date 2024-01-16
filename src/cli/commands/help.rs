// cli/commands/help.rs

use super::command_trait::Command;
use crate::cli::colors;
use async_trait::async_trait;

pub struct HelpCommand;

#[async_trait]
impl Command for HelpCommand {
    async fn run(&self,args:&str) {
        println!("{}|     {}      {}EZ HELP{}                                       {}            |{}",colors::WHITE,colors::RESET,colors::UNDERLINE,colors::RESET,colors::WHITE,colors::RESET);
        println!("{}|      -->  Use:{} {}-help{}                                    {}<--         |{}",colors::YELLOW,colors::RESET,colors::CYAN,colors::RESET,colors::YELLOW,colors::RESET);
        println!("{}|      -->  Use:{} {}-gpt{} [prompt]                            {}<--         |{}",colors::YELLOW,colors::RESET,colors::CYAN,colors::RESET,colors::YELLOW,colors::RESET);
        println!("{}|      -->  Use:{} {}-nmap{} [ip]                               {}<--         |{}",colors::YELLOW,colors::RESET,colors::CYAN,colors::RESET,colors::YELLOW,colors::RESET);
        println!("{}|      -->  Use:{} {}-rshell{} [create] [arg=none,base64]       {}<--         |{}",colors::YELLOW,colors::RESET,colors::CYAN,colors::RESET,colors::YELLOW,colors::RESET);
    }

    async fn helper(&self) {
        println!("Help Command: Usage information...");
    }
}
