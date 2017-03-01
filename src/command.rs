use clap::ArgMatches;

pub enum Command {
    Encode(u64),
    Decode(String),
}

pub fn from_args() -> Option<Command> {
    let matches = get_matches();
    
    if let Some(matches) = matches.subcommand_matches("encode") {
        match matches.value_of("value").and_then(|n| n.parse().ok()) {
            None => return None,
            Some(value) => return Some(Command::Encode(value)),
        }
    }

    if let Some(matches) = matches.subcommand_matches("decode") {
        return matches.value_of("value").map(|s| Command::Decode(s.to_string()))
    }

    return None
}

fn get_matches<'a>() -> ArgMatches<'a> {
    clap_app!(croc =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: "Does Crockford encoding")
        (@subcommand encode =>
            (about: "Encodes a u64 value")
            (@arg value: +required +takes_value "The value to be encoded")
        )
        (@subcommand decode =>
            (about: "Decodes a u64 value")
            (@arg value: +required +takes_value "The value to be decoded")
        )
    ).get_matches()
}
