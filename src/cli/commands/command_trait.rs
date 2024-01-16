// src/cli/commands/command_trait.rs
use async_trait::async_trait;

#[async_trait]
pub trait Command {
    async fn run(&self, args: &str);
    async fn helper(&self);
}
