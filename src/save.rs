use serde::{Deserialize, Serialize};

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
    NGF(ObjetNGF),
    Autre(),
}

#[derive(Serialize, Deserialize)]
struct ObjetNGF {
    matricule: String,
    id: u32,
}

#[derive(Serialize, Deserialize)]
struct Visite {}
