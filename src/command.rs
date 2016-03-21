pub enum Command<'a> {
    Pokemon(&'a str),
    Help(&'a str),
}

impl<'a> Command<'a> {
    pub fn from_str(cmd: &'a str) -> Command {
        let mut iter = cmd.split_whitespace();
        if let Some(word) = iter.next() {
            if word == "pokemon" {
                Command::Pokemon(&cmd[7..])
            } else if word == "p" {
                Command::Pokemon(&cmd[1..])
            } else if word == "help" {
                Command::Help(&cmd[4..])
            } else if word == "h" {
                Command::Help(&cmd[1..])
            } else {
                Command::Pokemon(cmd)
            }
        } else {
            Command::Pokemon(cmd)
        }
    }
}

pub fn print_help(helptext: &str) -> Vec<&'static str> {
    if helptext == "pokemon" || helptext == "p" {
        vec!["\"!dex pokemon name\": Search basic information about a Pokemon.",
            "The short form \"!dex p name\" may be used, or even omitted altogether, as in \"!dex name\".",
            "Searches may be by a Pokemon's name, e.g. \"!dex p drifloon\", or by National Dex number, e.g. \"!dex p 425\".",
            "Typing a search with no search text (i.e. \"!dex\" by itself) will recall your last search."]
    } else {
        vec!["Available commands: p[okemon], h[elp]",
            "See help specific to a command by typing \"!dex help command\"."]
    }
}
