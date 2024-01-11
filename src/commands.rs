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
                command_display::display();
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
                "task = \"{}\", state = \"td\", date = \"{}\"\n",
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
    use std::io::Write;
    use std::io::{Read, stdout};
    use crate::tasks::{ConversionError, State, Task};
    use crossterm::{ExecutableCommand, cursor, execute};
    use crossterm::terminal::{Clear, ClearType};

    pub fn get_tasks() -> Vec<Task>{
        const PATH: &str = "tdlist.txt";
        let mut file = File::open(PATH).unwrap();
        let mut buf = String::new();
        assert_ne!(file.read_to_string(&mut buf).unwrap(), 0);
        let mut data: Vec<Vec<&str>> = buf
            .split("\n")
            .map(|task| {
                task.split(",")
                    .map(|element| element.trim())
                    .collect::<Vec<&str>>()
            })
            .collect();
        data.pop();
        let mut formatted_data: Vec<Task> = vec![];
        for i in 0..data.len() {
            formatted_data.push(to_task(&data[i]).unwrap());
        }
        formatted_data
    }

    pub fn display() {
        let mut stdout = stdout();
        stdout.execute(Clear(ClearType::All)).unwrap();
        execute!(stdout, cursor::MoveTo(0, 0), cursor::Hide).unwrap();

        let formatted_data = get_tasks();
        print!("Date\t\tTask\t\tState\n");
        print!("----\t\t----\t\t-----");
        formatted_data.iter().for_each(|task| {
            let date = match task.date.as_str() {
                "" => "None\t".to_string(),
                _ => task.date.to_owned(),
            };
            execute!(stdout, cursor::MoveDown(1), cursor::MoveToColumn(0)).unwrap();
            write!(stdout, "{}", date).unwrap();
            stdout.execute(cursor::MoveToColumn(16)).unwrap();
            write!(stdout, "{}", task.id).unwrap();
            stdout.execute(cursor::MoveToColumn(32)).unwrap();
            write!(stdout, "{:?}", task.state).unwrap();
        });
        println!("\n");

        stdout.execute(cursor::Show).unwrap();
    }


    /// Converti le Vec<&str> en un objet Task
    pub fn to_task(input: &Vec<&str>) -> Result<Task, ConversionError> {
        let mut task = Task::default();
        if input.len() != 3 { return Err(ConversionError::BadLen); }
        for i in 0..input.len() {
            match input[i] {
                s if s.starts_with("task = \"") && s.ends_with("\"") => {
                    let name = &s["task = \"".len()..s.len() - 1];
                    task.id = name.to_string();
                }
                s if s.starts_with("state = \"") && s.ends_with("\"") => {
                    let state = &s["state = \"".len()..s.len() - 1];
                    task.state = match state {
                        "td" => State::Todo,
                        "ip" => State::InProgress,
                        "d" => State::Done,
                        _ => {
                            return Err(ConversionError::UnvalidArgForm); }
                    };
                }
                s if s.starts_with("date = \"") && s.ends_with("\"") => {
                    let date = &s["date = \"".len()..s.len() - 1];
                    task.date = date.to_string();
                }
                _ => {
                    return Err(ConversionError::UnvalidArgForm); }
            }
        }
        Ok(task)
    }
}

mod command_edit {}
