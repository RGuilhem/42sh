use ftsh::settings::Settings;
use std::io;
use std::io::Write;

fn main() {
    let settings = Settings::init();
    dbg!(settings);

    loop {
        let mut input = String::new();
        print!("$ ");
        let _ = io::stdout().flush();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        print!("{}", input);
        match input.trim() {
            "exit" => break,
            _ => (),
        }
    }
}
