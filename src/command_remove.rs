use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use crate::file_to_var::{delete_line, search_for_task};

pub fn remove(args: &Vec<String>) {
    if args.len() != 1 { panic!("Wrong number of args in rm command") }
    delete_line(search_for_task(args[0].as_str())).unwrap();

    let filename = Path::new("./tdlist.txt");
    let file = File::open(&filename).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> =
        reader
            .lines()
            .into_iter()
            .inspect(|line| println!("1. {:?}", line))  // très pratique la méthode inspect()
            .filter(|line| line.as_ref().unwrap().ne(""))
            .inspect(|line| println!("2. {:?}", line))
            .map(|line| { line.unwrap() })
            .collect();

    let mut file = File::options().write(true).open(filename).unwrap();
    lines.iter().for_each(|line| file.write_all(format!("{}\n", line).as_ref()).unwrap());
}
