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
            CommandType::Add => command_add::add(&self.args),
            CommandType::Remove => {}
            CommandType::Display => command_display::display(),
            CommandType::Edit => command_edit::edit(&self.args),
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
            _ => panic!("Unvalid Arg"),
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

    pub fn get_tasks() -> Vec<Task> {
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
        execute!(stdout, cursor::MoveTo(0, 1), cursor::Hide).unwrap();

        let formatted_data = get_tasks();

        print!(" State  Task\t\t\t\tDate\n");
        print!("-------|----\t\t\t\t----------");
        formatted_data.iter().for_each(|task| {
            let date = match task.date.as_str() {
                "" => "None\t".to_string(),
                _ => task.date.to_owned(),
            };
            execute!(stdout, cursor::MoveDown(1), cursor::MoveToColumn(0)).unwrap();
            write!(stdout, "{}", task.state).unwrap();
            stdout.execute(cursor::MoveToColumn(7)).unwrap();
            write!(stdout, "|{}", task.id).unwrap();
            stdout.execute(cursor::MoveToColumn(40)).unwrap();
            write!(stdout, "{}", date).unwrap();
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
                            return Err(ConversionError::UnvalidArgForm);
                        }
                    };
                }
                s if s.starts_with("date = \"") && s.ends_with("\"") => {
                    let date = &s["date = \"".len()..s.len() - 1];
                    task.date = date.to_string();
                }
                _ => {
                    return Err(ConversionError::UnvalidArgForm);
                }
            }
        }
        Ok(task)
    }
}

pub mod command_edit {
    use crate::commands::command_display::{get_tasks};

    pub fn edit(args: &Vec<String>) {
        match args[0].as_str() {
            "--task" | "-t" => edit_task(&args[0..args.len()].to_owned()),
            "--list" | "-l" => {}
            _ => panic!("Unvalid arg"),
        }
    }

    use std::fs::File;
    use std::io;
    use std::io::{BufRead, BufReader, Read, Seek, SeekFrom, Write};
    use std::path::Path;
    use std::str::from_utf8;
    use crate::commands::command_display;
    use crate::tasks::State;

    pub fn edit_task(args: &Vec<String>) {
        if args.len() < 3 { panic!("\nToo few arguments\n") }

        match args[2].as_str() {
            "--name" | "-id" | "-n" => edit_task_name(&args),
            "state" | "-s" => edit_task_state(&args),
            _ => {}
        }
    }

    pub fn edit_task_name(args: &Vec<String>) {
        let line = search_for_task(args[1].as_str());
        let mut file = File::options().write(true).read(true).open("./tdlist.txt").unwrap();

        if line == 0 {
            file.seek(SeekFrom::Start(get_nb_bytes_to_line(line))).unwrap();
        } else {
            file.seek(SeekFrom::Start(get_nb_bytes_to_line(line) + 1)).unwrap();
        };

        let task = &get_tasks()[line as usize];
        let state = match task.state {
            State::Todo => "td",
            State::InProgress => "ip",
            State::Done => "d"
        };
        file.write(format!("task = \"{}\", state = \"{}\", date = \"{}\"\n", args[3].as_str(), state, task.date).as_ref()).unwrap();
        command_display::display();
    }

    pub fn edit_task_state(args: &Vec<String>) {
        let line = search_for_task(args[1].as_str());
        let mut file = File::options().write(true).read(true).open("./tdlist.txt").unwrap();
        let offset = if line == 0 {
            file.seek(SeekFrom::Start(get_nb_bytes_to_line(line))).unwrap()
        } else {
            file.seek(SeekFrom::Start(get_nb_bytes_to_line(line) + 1)).unwrap()
        };

        let task = &get_tasks()[line as usize];
        let state = match args[3].to_lowercase().as_str() {
            "todo" | "td" => "td",
            "inprogress" | "ip" => "ip",
            "done" | "d" => "d",
            _ => panic!("Unvalid state argument")
        };

        // Obligé de faire des filouteries pour y arriver, j'y ai passé l'aprem
        delete_line(line).unwrap();
        file.seek(SeekFrom::Start(offset)).unwrap();
        let buffer = &mut vec![];
        file.read_to_end(buffer).unwrap();
        file.seek(SeekFrom::Start(offset)).unwrap();

        file.write(format!("task = \"{}\", state = \"{}\", date = \"{}\"{}", task.id, state, task.date, from_utf8(buffer).unwrap()).as_ref()).unwrap();
        command_display::display();
    }

    pub fn get_nb_bytes_to_line(line: u16) -> u64 {
        let mut file = File::options().append(true).read(true).open("./tdlist.txt").unwrap();
        let max = file.seek(SeekFrom::End(0)).unwrap();
        file.seek(SeekFrom::Start(0)).unwrap();
        let mut line_cnt: u64 = 0;
        let mut cnt: u64 = 0;
        loop {
            let mut buf = [0];
            file.read_exact(&mut buf).unwrap();
            if buf[0] == "\n".bytes().next().unwrap() { line_cnt += 1; }
            if line_cnt == line as u64 { break; }
            if file.seek(SeekFrom::Current(0)).unwrap() == max { return 0; }
            cnt += 1;
        }
        return cnt;
    }

    pub fn search_for_task(id: &str) -> u16 {
        let list = get_tasks();
        for i in 0..list.len() {
            if &list[i].id == &id.to_string() { return i as u16; }
        }
        panic!("Task not found")
    }

    pub fn delete_line(n_line: u16) -> io::Result<()> {
        let filename = Path::new("./tdlist.txt");

        let file = File::open(&filename)?;
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

        let line_to_delete = lines.get(n_line as usize).ok_or(io::Error::from(io::ErrorKind::NotFound))?;

        let mut file = File::create(&filename)?;
        for line in &lines {
            if line != line_to_delete {
                writeln!(file, "{}", line)?;
            } else {
                write!(file, "\n")?;
            }
        }

        Ok(())
    }
}