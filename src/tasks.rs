use std::fmt::{Display, Formatter};
use crossterm::style::Stylize;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Task {
    pub id: String,
    pub state: State,
    pub date: String,
}
impl Task {
    pub fn default() -> Task {
        Task {
            id: String::new(),
            state: State::Todo,
            date: String::new()
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum State {
    Todo,
    InProgress,
    Done,
}
impl Display for State {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        Ok(match self {
            State::Todo => {
                let style = "   X   ".on_dark_red();
                print!("{}", style)
            },
            State::InProgress => {
                let style = "  ...  ".on_dark_yellow();
                print!("{}", style)
            },
            State::Done => {
                let style = "   ✔️  ".on_green();
                print!("{}", style);
            }
        })
    }
}


#[derive(Debug)]
pub enum ConversionError {
    UnvalidArgForm,
    BadLen
}
