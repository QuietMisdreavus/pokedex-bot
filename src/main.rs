extern crate csv;
extern crate rustc_serialize;
extern crate irc;

mod env;
mod misc;
mod command;

use std::path::Path;
use std::collections::HashMap;
use env::Env;
use command as botcmd;

use irc::client::prelude::*;

fn main() {
    let irc_config = Config::load("config.json").unwrap();
    let csv_path = irc_config.get_option("csv_path").to_owned();
    let base_dir = Path::new(&csv_path);

    let db = Env::from_csv_path(&base_dir);

    println!("Connecting...");

    let srv = IrcServer::from_config(irc_config).unwrap();
    srv.identify().unwrap();

    println!("Ready!");

    // map: nick -> last id searched
    let mut last_search = HashMap::new();

    for message in srv.iter() {
        let message = message.unwrap();
        match message.command {
            Command::PRIVMSG(ref target, ref msg) => if msg.len() >= 4 {
                if misc::starts_with(msg.trim(), "!dex") {
                    if let Some(nick) = message.source_nickname() {
                        let body = &msg[4..];
                        println!("{}: {}: {}", target, nick, msg);
                        match botcmd::Command::from_str(body.trim()) {
                            botcmd::Command::Pokemon(name) => if name.trim().len() == 0 {
                                    // "!dex": show user's last search if available
                                    if let Some(id) = last_search.get(nick) {
                                        srv.send_privmsg(target, &db.print_poke(id)).unwrap();
                                    } else {
                                        srv.send_privmsg(target, &format!("Sorry, {}, I don't have a search on file for you.", nick)).unwrap();
                                    }
                                } else if let Some(id) = db.get_id(name.trim()) {
                                    // "!dex (name|number)": pokemon search
                                    srv.send_privmsg(target, &db.print_poke(&id)).unwrap();
                                    last_search.insert(nick.to_owned(), id);
                                } else {
                                    // "!dex (unknown)"
                                    srv.send_privmsg(target, "Sorry, that's not a pokemon I know of.").unwrap();
                                },
                            botcmd::Command::Help(helptext) => {
                                for txt in &botcmd::print_help(helptext.trim()) {
                                    srv.send_privmsg(target, txt).unwrap();
                                }
                            }
                        }
                    }
                } else if msg.trim() == "!help" {
                    srv.send_privmsg(target, "Type \"!dex name\" to search for information about a Pokemon, or \"!dex help\" for more commands.").unwrap();
                }
            },
            Command::NICK(ref new_nick) => {
                if let Some(nick) = message.source_nickname() {
                    println!("Nick change: {} -> {}", nick, new_nick);
                    if let Some(id) = last_search.remove(nick) {
                        last_search.insert(new_nick.to_owned(), id);
                    }
                }
            },
            _ => ()
        }
    }
}
