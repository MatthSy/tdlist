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
    "display" => Ok(Command {
      command: CommandType::Display,
      args: r_args,
    }),
    "edit" => Ok(Command {
      command: CommandType::Edit,
      args: r_args,
    }),
    "list" => Ok(Command {
      command: CommandType::List,
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
      CommandType::Add => {
        command_add::add(&self.args);
      }
      CommandType::Remove => {}
      CommandType::Display => {
        command_display::display(&self.args);
      }
      CommandType::Edit => {}
      CommandType::List => {}
      CommandType::Move => {}
    }
  }
}

mod command_add {
  use std::fs::{create_dir, File};
  use std::io::Write;

  pub fn add(args: &Vec<String>) {
    match args[0].as_str() {
      "--list" | "-l" => create_list(&args),
      "--task" | "-t" => create_task(&args),
      _ => panic!("\nUnknown parameter\nValid parameters :\n --list (alias : -l)\n --task (alias : -t)\n"),
    };
  }

  pub fn create_list(args: &Vec<String>) {
    println!("New list");
    create_dir(format!("{}", args[1])).unwrap();
    File::create(format!("{}/tdlist.txt", args[1])).unwrap();
    print!("List successfully created, move to dir to use the list");
  }

  pub fn create_task(args: &Vec<String>) {
    println!("New task");
    let mut file = File::options().append(true).open("tdlist.txt").unwrap();
    file.write_all(
      format!(
        "task = \"{}\", state = \"0\", date = \"{}\"\n",
        args[1],
        get_date(args)
      )
          .as_bytes(),
    )
        .unwrap();
    print!("Task successfully created");
  }

  pub fn get_date(args: &Vec<String>) -> String {
    if args.len() <= 3 {
      return String::new();
    }
    match args[2].as_str() {
      "--date" | "-d" => args[3].to_string(),
      _ => String::from("unvalid arg"),
    }
  }
}

mod command_display {
  use std::fs::File;
  use std::io::Read;

  #[allow(unused_variables)]
  pub fn display(args: &Vec<String>) {
    const PATH: &str = "tdlist.txt";
    let mut file = File::open(PATH).unwrap();
    let mut buf = String::new();
    assert_ne!(file.read_to_string(&mut buf).unwrap(), 0);
    println!("\n{}", buf);
    let data: Vec<Vec<&str>> = buf
        .split("\n")
        .map(|task| {
          task.split(",")
              .map(|element| element.trim())
              .collect::<Vec<&str>>()
        })
        .collect();
    // TODO : Compléter le code pour récupérer les variables des task dans le code
    dbg!(&data);
  }
}

mod command_edit {}
