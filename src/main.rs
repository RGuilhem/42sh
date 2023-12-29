use ftsh::parsing;
use std::env;
use std::io;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);

    loop {
        let mut input = String::new();
        print!("$ ");
        let _ = io::stdout().flush();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        print!("{}", input);
        let tokens = parsing::tokenize_input(&input);
        match input.trim() {
            "exit" => break,
            _ => (),
        }
    }
}
