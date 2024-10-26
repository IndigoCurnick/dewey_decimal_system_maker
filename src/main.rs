use std::{fs, path::PathBuf, str::FromStr};

use clap::Parser;
use serde::Deserialize;
use serde_json::{Map, Value};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    pub location: String,
}

fn main() {
    let cli = Cli::parse();

    let path = PathBuf::from_str(&cli.location).unwrap();

    if !path.exists() {
        panic!("Path `{:?}` does not exist", path);
    }

    if !path.is_dir() {
        panic!("Path `{:?}` is not a folder", path);
    }

    let dds = fs::read_to_string("dds/dds.json").unwrap();

    let val: Value = serde_json::from_str(&dds).unwrap();

    if let Value::Object(object) = val {
        process_classes(&path, &object);
    } else {
        panic!("Not an object!");
    }
}

#[derive(Deserialize)]
struct Structure {
    pub name: String,
    pub subcategories: Option<Value>,
}

fn process_classes(path: &PathBuf, object: &Map<String, Value>) {
    for (key, value) in object.iter() {
        let structure: Structure = serde_json::from_value(value.clone()).unwrap();

        let folder_name = format!("{} - {}", key, structure.name);
        let folder_path = path.join(folder_name);
        fs::create_dir(&folder_path).unwrap();

        process_divisions(&folder_path, &structure.subcategories.as_ref().unwrap());
    }
}

fn process_divisions(path: &PathBuf, class: &Value) {
    if let Value::Object(object) = class {
        for (key, value) in object.iter() {
            let structure: Structure = serde_json::from_value(value.clone()).unwrap();

            let folder_name = format!("{} - {}", key, structure.name);
            let folder_path = path.join(folder_name);
            fs::create_dir(&folder_path).unwrap();

            process_sections(&folder_path, &structure.subcategories.as_ref().unwrap());
        }
    }
}

fn process_sections(path: &PathBuf, division: &Value) {
    if let Value::Object(object) = division {
        for (key, value) in object.iter() {
            let structure: Structure = serde_json::from_value(value.clone()).unwrap();

            let folder_name = format!("{} - {}", key, structure.name);
            let folder_path = path.join(folder_name);
            fs::create_dir(&folder_path).unwrap();
        }
    }
}
