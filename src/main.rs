#[macro_use]
extern crate clap;

extern crate crockford;

mod command;

use command::Command;

fn main() {
    match command::from_args() {
        Some(Command::Encode(n)) => println!("{}", crockford::encode(n)),
        Some(Command::Decode(n)) => {
            let value = crockford::decode(n).expect("Not a valid Crockford value.");
            println!("{}", value);
        }

        _ => {
            println!("usage: croc encode <positive integer> or croc decode <encoded string>");
            std::process::exit(1);
        }
    }
}
