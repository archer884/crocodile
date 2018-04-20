extern crate crockford;

mod command;

static USAGE: &str = r#"usage:
    croc encode <positve integer>
    croc decode <encoded string>"#;

fn main() {
    use command::Command;

    match Command::from_args() {
        Some(Command::Encode(inputs)) => do_encode(&inputs),
        Some(Command::Decode(inputs)) => do_decode(&inputs),

        _ => {
            println!("{}", USAGE);
            std::process::exit(1);
        }
    }
}

fn do_encode(inputs: &[u64]) {
    for &n in inputs {
        println!("{}", crockford::encode(n));
    }
}

fn do_decode(inputs: &[impl AsRef<str>]) {
    for n in inputs {
        match crockford::decode(n) {
            Err(e) => println!("ERROR: {}", e),
            Ok(n) => println!("{}", n),
        }
    }
}
