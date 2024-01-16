// cli/commands/nmap.rs
use super::command_trait::Command;
use async_trait::async_trait;
use std::process::Command as CliCommand;

pub struct NmapCommand;

#[async_trait]
impl Command for NmapCommand {
    async fn run(&self, args: &str) {
        // Spawn a blocking task to run the command
        let args_clone = args.to_owned() + " -sS -sV";
        let output = tokio::task::spawn_blocking(move || {
            CliCommand::new("nmap")
                .arg(&args_clone)
                .output()
        }).await.expect("Failed to spawn_blocking");

        // Now `output` is a Result of the command execution
        match output {
            Ok(output) if output.status.success() => {
                let output_str = String::from_utf8_lossy(&output.stdout);
                println!("Output: {}", output_str);
            },
            Ok(output) => {
                let error_str = String::from_utf8_lossy(&output.stderr);
                eprintln!("Error: {}", error_str);
            },
            Err(e) => eprintln!("Failed to execute command: {}", e),
        }
    }

    async fn helper(&self) {
        println!("Help Command: Usage information...");
    }
}
