use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
use std::io;
use std::path::Path;
use crate::command_display;
use crate::tasks::Task;

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
        formatted_data.push(command_display::to_task(&data[i]).unwrap());
    }
    formatted_data
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
