use std::fs::{create_dir, File};
use std::io::Write;

#[derive(Debug)]
pub struct Command {
  command: CommandType,
  args: Vec<String>
}

#[derive(Debug)]
pub enum CommandType {
  Add,
  Remove,
  Display,
  Edit,
  List,
  Move,
}

#[derive(Debug)]
pub struct CommandError {
  pub input: String,
}

#[allow(dead_code)]
pub fn display_args(args: &Vec<String>) {
  if args[0] == *"target\\debug\\tdlist.exe".to_string() {
    args.iter().cloned().enumerate().for_each(|(i, arg)| { println!("Arg {i}: {}", arg) });
  }
}

pub fn match_input_to_command(args: &Vec<String>) -> Result<Command, CommandError> {
  let r_args = args[2..args.len()].to_vec();
  match args[1].to_lowercase().as_str() {
    "add" | "a" => Ok(Command {command:CommandType::Add, args:r_args}),
    "remove" | "rm" => Ok(Command {command:CommandType::Remove, args:r_args}),
    "display" => Ok(Command {command:CommandType::Display, args:r_args}),
    "edit" => Ok(Command {command:CommandType::Edit, args:r_args}),
    "list" => Ok(Command {command:CommandType::List, args:r_args}),
    "move" => Ok(Command {command:CommandType::Move, args:r_args}),
    _ => Err(CommandError{input: args[1].clone()})
  }
}

impl Command {
  pub fn execute(&self) {
    match self.command {
      CommandType::Add => {
        command_add(&self.args);
      }
      CommandType::Remove => {}
      CommandType::Display => {}
      CommandType::Edit => {}
      CommandType::List => {}
      CommandType::Move => {}
    }
  }
}

pub fn command_add(args: &Vec<String>) {
  match args[0].as_str() {
    "--list" | "-l" => create_list(&args),
    "--task" | "-t" => create_task(&args),
    _ => panic!("\nUnknown parameter\nValid parameters :\n --list (alias : -l)\n --task (alias : -t)\n"),
  };
}

pub fn create_list(args: &Vec<String>) {
  println!("New list");
  create_dir(format!("{}", args[1])).unwrap();
  File::create(format!("{}/list.txt", args[1])).unwrap();
  print!("List successfully created, move to dir to use the list");
}
pub fn create_task(args: &Vec<String>) {
  println!("New task");
  let mut file = File::options().append(true).open("list.txt").unwrap();
  file.write_all(format!("task = \"{}\" state = \"0\"\n", args[1]).as_bytes()).unwrap();
  print!("Task successfully created");
}