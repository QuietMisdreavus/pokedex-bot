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
        vec!["Type \"!dex name\" or \"!dex number\" to search by name or National Dex number.",
            "The command \"!dex pokemon name\" or \"!dex p name\" may also be used.",
            "Typing a search with no search text (i.e. \"!dex\" by itself) will recall your last search."]
    } else {
        vec!["Available commands: p[okemon], h[elp]",
            "See help specific to a command by typing \"!dex help command\"."]
    }
}
