#[allow(dead_code)]
#[derive(Debug)]
pub struct Task {
    pub id: String,
    pub state: State,
    pub date: String,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum State {
    Todo,
    InProgress,
    Done,
}

#[derive(Debug)]
pub enum ConversionError {
    UnvalidArgForm,
    BadLen
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