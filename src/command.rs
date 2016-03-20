use misc;

pub enum Command<'a> {
    Pokemon(&'a str)
}

impl<'a> Command<'a> {
    pub fn from_str(cmd: &'a str) -> Command {
        if misc::starts_with(cmd, "pokemon") {
            Command::Pokemon(&cmd[7..])
        } else if misc::starts_with(cmd, "p") {
            Command::Pokemon(&cmd[1..])
        } else {
            Command::Pokemon(cmd)
        }
    }
}

