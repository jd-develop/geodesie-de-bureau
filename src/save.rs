use serde::{Deserialize, Serialize};

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
