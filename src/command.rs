pub enum Command {
    Encode(Vec<u64>),
    Decode(Vec<String>),
}

impl Command {
    pub fn from_args() -> Option<Self> {
        use std::env;
        use std::iter;

        let mut args = env::args().skip(1);
        let head = args.next()?;

        match &*head {
            "decode" => Some(Command::Decode(args.collect())),
            "encode" => {
                let input_values: Option<_> = args.map(|x| x.parse().ok()).collect();
                Some(Command::Encode(input_values?))
            }

            // If the user does not select a subcommand, assume they mean to encode something.
            // If it isn't a number, they can eat me.
            _ => {
                let input_values: Option<_> = iter::once(head)
                    .chain(args)
                    .map(|x| x.parse().ok()).collect();

                Some(Command::Encode(input_values?))
            }
        }
    }
}
