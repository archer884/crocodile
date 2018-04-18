extern crate crockford;

mod command;

static USAGE: &str = r#"usage:
    croc encode <positve integer>
    croc decode <encoded string>"#;

fn main() {
    use command::Command;

    match Command::from_args() {
        Some(Command::Encode(n)) => println!("{}", crockford::encode(n)),
        Some(Command::Decode(n)) => {
            let value = crockford::decode(n).expect("Not a valid Crockford value.");
            println!("{}", value);
        }

        _ => {
            println!("{}", USAGE);
            std::process::exit(1);
        }
    }
}
