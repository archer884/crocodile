pub enum Command {
    Encode(u64),
    Decode(String),
}

impl Command {
    pub fn from_args() -> Option<Self> {
        use std::env;

        let mut args = env::args().skip(1);
        match args.next()?.as_ref() {
            "encode" => Some(Command::Encode(args.next()?.parse().ok()?)),
            "decode" => Some(Command::Decode(args.next()?.parse().ok()?)),

            _ => None
        }
    }
}
