// pokedex-bot - an irc bot that relays information about pokemon
// Copyright (C) 2016 QuietMisdreavus
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

use std::path::Path;
use std::collections::HashMap;
use std::str::FromStr;
use csv;
use rand;

pub struct Env {
    type_map: HashMap<i32, (Option<i32>, Option<i32>)>,
    pub species_names: HashMap<i32, (String, String)>,
    pub name_lookup: HashMap<String, i32>,
    type_names: HashMap<i32, String>
}

impl Env {
    pub fn from_csv_path(base_dir: &Path) -> Env {
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
        let mut type_map : HashMap<i32, (Option<i32>, Option<i32>)> = HashMap::new();

        for t_row in type_rdr.decode() {
            if let Ok((pokemon_id, type_id, slot)) = t_row {
                let type_val : (Option<i32>, Option<i32>) = match slot {
                    1 => (Some(type_id), None),
                    2 => (None, Some(type_id)),
                    _ => unreachable!()
                };

                let pokemon_types = type_map.entry(pokemon_id).or_insert((None, None));

                *pokemon_types = (pokemon_types.0.or(type_val.0), pokemon_types.1.or(type_val.1));
            }
        }

        Env {
            type_map: type_map,
            species_names: species_names,
            name_lookup: name_lookup,
            type_names: type_names
        }
    }

    pub fn pokemon_types<'a, 'b>(&'a self, id: &'b i32) -> (Option<&'a str>, Option<&'a str>) {
        match self.type_map.get(id) {
            Some(&(t1, t2)) => (t1.map(|t_id| self.type_names.get(&t_id).unwrap().as_str()),
                                t2.map(|t_id| self.type_names.get(&t_id).unwrap().as_str())),
            None => (Some("Fake"), None)
        }
    }

    pub fn print_poke(&self, id: &i32) -> String {
        let s_name = self.species_names.get(id).unwrap();
        let types = self.pokemon_types(id);

        let t_name = match types {
            (Some(t1), Some(t2)) => format!("{}/{}", t1, t2),
            (Some(t1), None) => t1.to_owned(),
            (None, _) => unreachable!()
        };

        format!("#{} {}, The {} Pokemon, {} type", id, s_name.0, s_name.1, t_name)
    }

    pub fn get_id(&self, search: &str) -> Option<i32> {
        if let Ok(id) = i32::from_str(search) {
            if self.species_names.contains_key(&id) {
                Some(id)
            } else {
                None
            }
        } else if let Some(id) = self.name_lookup.get(&search.to_lowercase()) {
            Some(*id)
        } else {
            None
        }
    }

    pub fn get_random_id(& self) -> & i32 {
        let mut rng = rand::thread_rng();
        rand::sample(&mut rng, self.species_names.keys(), 1).pop().unwrap()
    }
}

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
