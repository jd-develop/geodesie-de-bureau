use serde::{Deserialize, Serialize};
use shellexpand;
use std::{error::Error, fs, result::Result, string::String};

use crate::ngf;

/// This struct should be able to serialize and deserialize a JSON save
#[derive(Serialize, Deserialize)]
struct SaveJSON {
    options: Options,
    objets: Vec<Objet>,
    visites: Vec<Visite>,
}

#[derive(Serialize, Deserialize)]
struct Options {}

#[derive(Serialize, Deserialize)]
enum Objet {
    NGF(ngf::json_mappings::repere::RepèreNivellement),
    Autre(),
}

#[derive(Serialize, Deserialize)]
struct Visite {}

fn determine_config_directory() -> Result<String, String> {
    match std::env::consts::OS {
        "ios" | "android" => {
            Err("Sorry, 'ios' and 'android' are not supported by Géodésie de Bureau".to_string())
        }

        "macos" => {
            Ok(shellexpand::tilde("~/Library/Preferences/org.jd-develop.geodesie/").to_string())
        }

        "windows" => {
            let appdata = match std::env::var("APPDATA") {
                Ok(value) => value,
                Err(err) => return Err(err.to_string()),
            };
            Ok(format!("{}\\jd-develop\\geodesie", appdata).to_string())
        }

        _ => Ok(shellexpand::tilde("~/.config/jd-develop/geodesie").to_string()),
    }
}

/// This function writes the data in the save argument on the disk
fn write_store(save: &SaveJSON) -> Result<(), Box<dyn Error>> {
    let config_dir_path = determine_config_directory()?;
    fs::create_dir_all(&config_dir_path)?;
    fs::write(config_dir_path + "/save.json", serde_json::to_string(save)?)?;
    Ok(())
}

/// This function reads the data on the disk and returns a SaveJSON
fn read_store() -> Result<SaveJSON, Box<dyn Error>> {
    let config_dir_path = determine_config_directory()?;
    fs::create_dir_all(&config_dir_path)?; // Just to avoid some errors ;)
    let read_to_string: Result<String, std::io::Error> = fs::read_to_string(config_dir_path + "/save.json");
    let from_str: Result<SaveJSON, serde_json::Error> = serde_json::from_str::<SaveJSON>(read_to_string?.as_str());
    Ok(from_str?)
}
