use crate::command_add::command_add;

#[derive(Debug)]
pub struct Command {
    command: CommandType,
    args: Vec<String>,
}

#[derive(Debug)]
pub enum CommandType {
    Add,
    Remove,
    Display,
    Edit,
    Help,
    Move,
}

#[derive(Debug)]
pub struct CommandError {
    pub input: String,
}

#[allow(dead_code)]
pub fn display_args(args: &Vec<String>) {
    if args[0] == *"target\\debug\\tdlist.exe".to_string() {
        args.iter()
            .cloned()
            .enumerate()
            .for_each(|(i, arg)| println!("Arg {i}: {}", arg));
    }
}

pub fn match_input_to_command(args: &Vec<String>) -> Result<Command, CommandError> {
    let r_args = args[2..args.len()].to_vec();
    match args[1].to_lowercase().as_str() {
        "add" | "a" => Ok(Command {
            command: CommandType::Add,
            args: r_args,
        }),
        "remove" | "rm" => Ok(Command {
            command: CommandType::Remove,
            args: r_args,
        }),
        "display" | "d" => Ok(Command {
            command: CommandType::Display,
            args: r_args,
        }),
        "edit" | "e" => Ok(Command {
            command: CommandType::Edit,
            args: r_args,
        }),
        "help" | "h" => Ok(Command {
            command: CommandType::Help,
            args: r_args,
        }),
        "move" => Ok(Command {
            command: CommandType::Move,
            args: r_args,
        }),
        _ => Err(CommandError {
            input: args[1].clone(),
        }),
    }
}

impl Command {
    pub fn execute(&self) {
        match self.command {
            CommandType::Add => command_add::add(&self.args),
            CommandType::Remove => crate::command_remove::remove(&self.args),
            CommandType::Display => crate::command_display::display(),
            CommandType::Edit => crate::command_edit::edit(&self.args),
            CommandType::Help => {}
            CommandType::Move => {}
        }
    }
}