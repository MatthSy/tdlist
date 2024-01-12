pub mod command_add {
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
