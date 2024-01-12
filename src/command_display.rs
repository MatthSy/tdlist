use std::io::stdout;
use crossterm::terminal::{Clear, ClearType};
use crossterm::{cursor, ExecutableCommand, execute};
use crate::file_to_var;
use crate::tasks::{ConversionError, State, Task};

pub fn display() {
    let mut stdout = stdout();
    stdout.execute(Clear(ClearType::All)).unwrap();
    execute!(stdout, cursor::MoveTo(0, 1), cursor::Hide).unwrap();

    let formatted_data = file_to_var::get_tasks();

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
