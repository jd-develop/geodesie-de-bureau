use serde::{Deserialize, Serialize};
use std::string::String;
use std::result::Result;
extern crate shellexpand;

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
        "ios" | "android" => Err(
            "Sorry, 'ios' and 'android' are not supported by Géodésie de Bureau"
                .to_string()
        ),

        "macos" => Ok(shellexpand::tilde("~/Library/Preferences/org.jd-develop.geodesie/").to_string()),

        "windows" => {
            let appdata = match std::env::var("APPDATA") {
                Ok(value) => value,
                Err(err) => return Err(err.to_string()) 
            };
            Ok(
                format!("{}\\jd-develop\\geodesie", appdata)
                .to_string()
            )
        },

        _ => Ok(shellexpand::tilde("~/.config/jd-develop/geodesie").to_string()),
    }

}

