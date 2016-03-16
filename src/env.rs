use std::path::Path;
use std::collections::HashMap;
use csv;

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
