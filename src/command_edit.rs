use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::str::from_utf8;
use crate::file_to_var;
use crate::file_to_var::get_tasks;
use crate::tasks::State;

pub fn edit(args: &Vec<String>) {
    match args[0].as_str() {
        "--task" | "-t" => edit_task(&args[0..args.len()].to_owned()),
        "--list" | "-l" => {}
        _ => panic!("Unvalid arg"),
    }
}

pub fn edit_task(args: &Vec<String>) {
    if args.len() < 3 { panic!("\nToo few arguments\n") }

    match args[2].as_str() {
        "--name" | "-id" | "-n" => edit_task_name(&args),
        "state" | "-s" => edit_task_state(&args),
        _ => {}
    }
}

pub fn edit_task_name(args: &Vec<String>) {
    let line = file_to_var::search_for_task(args[1].as_str());
    let mut file = File::options().write(true).read(true).open("./tdlist.txt").unwrap();

    let offset = if line == 0 {
        file.seek(SeekFrom::Start(file_to_var::get_nb_bytes_to_line(line))).unwrap()
    } else {
        file.seek(SeekFrom::Start(file_to_var::get_nb_bytes_to_line(line) + 1)).unwrap()
    };

    let task = &get_tasks()[line as usize];
    let state = match task.state {
        State::Todo => "td",
        State::InProgress => "ip",
        State::Done => "d"
    };

    // Obligé de faire des filouteries pour y arriver, j'y ai passé l'aprem
    file_to_var::delete_line(line).unwrap();
    file.seek(SeekFrom::Start(offset)).unwrap();
    let buffer = &mut vec![];
    file.read_to_end(buffer).unwrap();
    file.seek(SeekFrom::Start(offset)).unwrap();

    file.write(format!("task = \"{}\", state = \"{}\", date = \"{}\"{}", args[3].as_str(), state, task.date, from_utf8(buffer).unwrap()).as_ref()).unwrap();
    crate::command_display::display();
}

pub fn edit_task_state(args: &Vec<String>) {
    let line = file_to_var::search_for_task(args[1].as_str());
    let mut file = File::options().write(true).read(true).open("./tdlist.txt").unwrap();
    let offset = if line == 0 {
        file.seek(SeekFrom::Start(file_to_var::get_nb_bytes_to_line(line))).unwrap()
    } else {
        file.seek(SeekFrom::Start(file_to_var::get_nb_bytes_to_line(line) + 1)).unwrap()
    };

    let task = &get_tasks()[line as usize];
    let state = match args[3].to_lowercase().as_str() {
        "todo" | "td" => "td",
        "inprogress" | "ip" => "ip",
        "done" | "d" => "d",
        _ => panic!("Unvalid state argument")
    };

    // Obligé de faire des filouteries pour y arriver, j'y ai passé l'aprem
    file_to_var::delete_line(line).unwrap();
    file.seek(SeekFrom::Start(offset)).unwrap();
    let buffer = &mut vec![];
    file.read_to_end(buffer).unwrap();
    file.seek(SeekFrom::Start(offset)).unwrap();

    file.write(format!("task = \"{}\", state = \"{}\", date = \"{}\"{}", task.id, state, task.date, from_utf8(buffer).unwrap()).as_ref()).unwrap();
    crate::command_display::display();
}
