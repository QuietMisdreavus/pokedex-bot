extern crate csv;
extern crate rustc_serialize;
extern crate irc;

mod env;

use std::path::Path;
use std::collections::HashMap;
use std::str::FromStr;
use env::Env;

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
                if let Some(nick) = message.source_nickname() {
                    let (cmd, body) = msg.split_at(4);
                    if cmd == "!dex" {
                        println!("{}: {}: {}", target, nick, msg);
                        if body.trim().len() == 0 {
                            // "!dex": show user's last search if available
                            if let Some(id) = last_search.get(nick) {
                                srv.send_privmsg(target, &db.print_poke(id)).unwrap();
                            }
                        } else if let Some(id) = get_id(body.trim(), &db) {
                            // "!dex (name|number)": pokemon search
                            srv.send_privmsg(target, &db.print_poke(&id)).unwrap();
                            last_search.insert(nick.to_owned(), id);
                        } else {
                            // "!dex (unknown)"
                            srv.send_privmsg(target, "Sorry, that's not a pokemon I know of.").unwrap();
                        }
                    }
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

fn get_id(search: &str, db: &Env) -> Option<i32> {
    if let Ok(id) = i32::from_str(search) {
        if db.species_names.contains_key(&id) {
            Some(id)
        } else {
            None
        }
    } else if let Some(id) = db.name_lookup.get(&search.to_lowercase()) {
        Some(id.clone())
    } else {
        None
    }
}
