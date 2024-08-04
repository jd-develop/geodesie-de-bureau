use serde::{Deserialize, Serialize};
use super::bbox::*;

// TODO Implement std::fmt::Display and std::fmt::Debug traits properly
#[derive(Serialize, Deserialize)]
pub struct RepèreNivellement {
    pub matricule: String,
    pub cid: i64,
    pub fiche_url: String,
    pub systeme_altimetrique: NivfReaCode,
    pub altitude: String,
    pub altitude_complementaire: String,
    pub altitude_type: i64,

    pub derniere_observation: String,
    pub nouveau_calcul: String,
    pub derniere_visite: String,
    pub etat: RnÉtatCode,

    pub rn_type: RnTypeCode,
    pub type_complement: Option<String>,
    pub canex_info: String,
    pub type_complement_avec_canex: String,

    pub longitude: f64,
    pub latitude: f64,
    pub e: String,
    pub n: String,

    pub departement: String,
    pub insee: String,
    pub commune: String,
    pub voie_suivie: String,
    pub voie_de: Option<String>,
    pub voie_vers: Option<String>,
    pub voie_cote: VoieCôtéCode,
    pub voie_pk: Option<String>,
    pub distance: Option<String>,
    pub du_repere: String,
    pub localisation: Option<String>,

    pub support: String,
    pub partie_support: Option<String>,
    pub reperement_horizontal: Option<String>,
    pub reperement_vertical: Option<String>,

    pub hors_ign: String,
    pub remarques: String,
    pub exploitabilite_gps: RnGpsEploitCode,
}
