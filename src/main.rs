extern crate csv;
extern crate rustc_serialize;
extern crate irc;

use std::path::Path;
use std::collections::HashMap;
use std::str::FromStr;

use irc::client::prelude::*;

#[derive(RustcDecodable)]
struct SpeciesName {
    species_id: i32,
    lang_id: i32,
    name: String,
    genus: String
}

#[derive(RustcDecodable)]
struct TypeName {
    type_id: i32,
    lang_id: i32,
    name: String
}

struct Env<'a> {
    type_map: HashMap<i32, (Option<&'a str>, Option<&'a str>)>,
    species_names: HashMap<i32, (String, String)>,
    name_lookup: HashMap<String, i32>
}

fn main() {
    let irc_config = Config::load("config.json").unwrap();
    let csv_path = irc_config.get_option("csv_path").to_owned();
    let base_dir = Path::new(&csv_path);

    let mut name_rdr = csv::Reader::from_file(base_dir.join("pokemon_species_names.csv")).unwrap().has_headers(true);
    let mut type_name_rdr = csv::Reader::from_file(base_dir.join("type_names.csv")).unwrap().has_headers(true);

    let mut type_names = HashMap::new();

    for n_row in type_name_rdr.decode::<TypeName>().filter_map(|n| n.ok()).filter(|n| n.lang_id == 9) {
        type_names.insert(n_row.type_id, n_row.name);
    }

    let mut species_names = HashMap::new();
    let mut name_lookup = HashMap::new();

    for s_row in name_rdr.decode::<SpeciesName>().filter_map(|n| n.ok()).filter(|n| n.lang_id == 9) {
        name_lookup.insert(s_row.name.to_lowercase(), s_row.species_id);
        species_names.insert(s_row.species_id, (s_row.name, s_row.genus));
    }

    let mut type_rdr = csv::Reader::from_file(base_dir.join("pokemon_types.csv")).unwrap().has_headers(true);
    let mut type_map : HashMap<i32, (Option<&str>, Option<&str>)> = HashMap::new();

    for t_row in type_rdr.decode() {
        if let Ok((pokemon_id, type_id, slot)) = t_row {
            let t_name = type_names.get(&type_id).unwrap();

            let type_val : (Option<&str>, Option<&str>) = match slot {
                1 => (Some(t_name), None),
                2 => (None, Some(t_name)),
                _ => unreachable!()
            };

            let pokemon_types = type_map.entry(pokemon_id).or_insert((None, None));

            *pokemon_types = (pokemon_types.0.or(type_val.0), pokemon_types.1.or(type_val.1));
        }
    }

    let db = Env {
        type_map: type_map,
        species_names: species_names,
        name_lookup: name_lookup
    };

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
                                srv.send_privmsg(target, &print_poke(id, &db)).unwrap();
                            }
                        } else if let Some(id) = get_id(body.trim(), &db) {
                            // "!dex (name|number)": pokemon search
                            srv.send_privmsg(target, &print_poke(&id, &db)).unwrap();
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

fn print_poke(id: &i32, db: &Env) -> String {
    let s_name = db.species_names.get(id).unwrap();
    let types = db.type_map.get(id).unwrap();

    let t_name = match types {
        &(Some(t1), Some(t2)) => format!("{}/{}", t1, t2),
        &(Some(t1), None) => t1.into(),
        &(None, _) => unreachable!()
    };

    format!("#{} {}, The {} Pokemon, {} type", id, s_name.0, s_name.1, t_name)
}
