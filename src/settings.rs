use std::env;

#[derive(Debug)]
pub struct Options {
    c: bool,
    i: bool,
    s: bool,
}

#[derive(Debug)]
pub struct Settings {
    options: Options,
}

impl Settings {
    fn default() -> Self {
        Self {
            options: Options {
                c: false,
                i: true,
                s: true,
            },
        }
    }

    pub fn init() -> Self {
        let args: Vec<String> = env::args().collect();
        let mut settings = Self::default();
        //reading options
        for (i, arg) in args.iter().enumerate() {
            println!("arg {i}: {:?}", arg);
        }
        settings
    }
}
