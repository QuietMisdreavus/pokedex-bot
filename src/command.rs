pub enum Command<'a> {
    Pokemon(&'a str)
}

impl<'a> Command<'a> {
    pub fn from_str(cmd: &'a str) -> Command {
        let mut iter = cmd.split_whitespace();
        if let Some(word) = iter.next() {
            if word == "pokemon" {
                Command::Pokemon(&cmd[7..])
            } else if word == "p" {
                Command::Pokemon(&cmd[1..])
            } else {
                Command::Pokemon(cmd)
            }
        } else {
            Command::Pokemon(cmd)
        }
    }
}

