use text_io::read;


fn clean_cmd(command: &str) -> String {
    command.replace("\r", "")
           .replace("\n", "")
}


fn parse_cmd(command: String) -> Option<i32> {
    let command: String = clean_cmd(command.as_str());
    let command_parts: Vec<&str> = command.split(" ").collect::<Vec<&str>>();

    println!("{command_parts:?}");

    if command == "exit".to_string() {
        Some(0)
    } else {
        None
    }
}


pub fn run_shell() -> i32 {
    let exit_code: i32 = loop {
        print!("rust-sub-cli>");

        let exit: Option<i32> = parse_cmd(read!("{}\n"));

        match exit {
            Some(code) => { break code; },
            None => {},
        }
    };

    exit_code
}