// pokedex-bot - an irc bot that relays information about pokemon
// Copyright (C) 2016 Bryan Mitchell (icesoldier)
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

pub enum Command<'a> {
    Pokemon(&'a str),
    Help(&'a str),
    Random,
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
            } else if word == "random" {
                Command::Random
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
    } else if helptext == "random" {
        vec!["\"!dex random\": Selects a random pokemon, then displays its information as if it had been searched for by \"!dex pokemon\"."]
    } else {
        vec!["Available commands: p[okemon], h[elp], random",
            "See help specific to a command by typing \"!dex help command\"."]
    }
}
