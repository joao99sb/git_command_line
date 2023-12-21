mod git;
use std::env;
use std::process::ExitCode;

use command_macro::*;
use git::*;

fn usage(program: &str) {
    eprintln!("Usage: {program} <command>");
    eprintln!("Commands:");
    for command in COMMANDS.iter() {
        eprintln!(
            "     {name} - {description} ",
            name = command.name,
            description = command.description
        );
    }
}

#[command("help", "Print this help message")]
fn help_command(program: &str, mut args: env::Args) -> ExitCode {
    if let Some(command_name) = args.next() {
        if let Some(command) = COMMANDS.iter().find(|command| command.name == command_name) {
            println!(
                "{name} - {description}",
                name = command.name,
                description = command.description
            )
        } else {
            eprintln!("ERROR: command {command_name} is not found");
            return ExitCode::FAILURE;
        }
    } else {
        usage(&program);
    }
    return ExitCode::SUCCESS;
}

#[command("cp", "add, commit and push with, pass flag -m to add a message")]
fn commit_and_push(program: &str, mut args: env::Args) -> ExitCode {
    if let Some(flag) = args.next() {
        let res = match flag.as_str() {
            "--m" => match args.next() {
                Some(message) => {
                    git_add().unwrap();
                    git_commit(&message).unwrap();
                    git_push().unwrap();
                    Ok(())
                }
                None => {
                    eprintln!("No message was provided");
                    Err(())
                }
            },
            _ => {
                eprintln!("flag {flag} is unknown");
                Err(())
            }
        };
        match res {
            Ok(_) => ExitCode::SUCCESS,
            Err(_) => ExitCode::FAILURE,
        }
    } else {
        eprintln!("No arguments was provided");
        ExitCode::FAILURE
    }
}

struct Command {
    name: &'static str,
    description: &'static str,
    run: fn(&str, env::Args) -> ExitCode,
}
const COMMANDS: &[Command] = command_list!();

fn main() -> ExitCode {
    let mut args = env::args();
    let program = args.next().expect("program");
    if let Some(command_name) = args.next() {
        let command = COMMANDS.iter().find(|command| command.name == command_name);

        match command {
            Some(command) => {
                return (command.run)(&program, args);
            }
            _ => {
                usage(&program);
                eprintln!("ERROR: No command {command_name} is unknown");
                return ExitCode::FAILURE;
            }
        }
    } else {
        usage(&program);
        eprintln!("ERROR: No command provided");
        return ExitCode::FAILURE;
    }
}
