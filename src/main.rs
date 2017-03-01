extern crate tw_chip8;

use std::env;
use std::process;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

enum Action {
    Run(Option<String>),
    Disassemble,
}

struct Config {
    action: Action,
    filename: String,
}

impl Config {
    fn new<T>(args: &mut T) -> Result<Config, &'static str> where T: Iterator<Item = String> {
        args.next();

        let mut path: Option<String> = None;
        let mut action = Action::Run(None);
        for arg in args {
            match &arg[..] {
                "--run" => action = Action::Run(None),
                "--disassemble" => action = Action::Disassemble,
                s if s.starts_with("--dump=") => {
                    match action {
                        Action::Run(ref mut path) => *path = Some(String::from(&s[7..])),
                        Action::Disassemble => return Err("Cannot dump on disassemble."),
                    }
                },
                s => path = Some(String::from(s)),
            };
        }

        match path {
            Some(p) => Ok(Config {
                action: action,
                filename: p,
            }),
            None => Err("ROM file needed.")
        }
    }
}

fn main() {
    let mut stderr = std::io::stderr();

    let config = Config::new(&mut env::args()).unwrap_or_else(|err| {
        writeln!(&mut stderr, "{}", err).expect("Cannot write to stderr.");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        writeln!(&mut stderr, "Application error: {}", e).expect("Cannot write to stderr.");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;
    let mut data: Vec<u8> = Vec::new();
    f.read_to_end(&mut data)?;

    match config.action {
        Action::Run(dump_file) => {
            let mut f = match dump_file {
                Some(path) => {
                    let file = File::create(path)?;
                    Some(file)
                },
                None => None,
            };
            tw_chip8::run(data, &mut f)
        },
        Action::Disassemble => Ok(tw_chip8::disassemble(data)),
    }
}