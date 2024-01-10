pub struct Task {
    id: String,
    state: State,
    date: String,
}

enum State {
    Todo,
    InProgress,
    Done,
}
