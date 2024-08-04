//! Can serialize / deserialize the output of the « https://geodesie.ign.fr/ripgeo/fr/api/nivrn/bbox/{long}/{lat}/json/ » API call
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BBox {
    #[serde(rename = "type")]
    pub bbox_type: String,
    pub features: Vec<Feature>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Feature {
    #[serde(rename = "type")]
    pub feature_type: FeatureType,
    pub geometry: Geometry,
    pub properties: Properties,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum FeatureType {
    Feature,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Geometry {
    #[serde(rename = "type")]
    pub geometry_type: GeometryType,
    pub coordinates: Vec<f64>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum GeometryType {
    Point,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Properties {
    pub image_name: String,
    pub rn_type_code: String,
    pub nivf_ref_en_code: i64,
    pub nivf_rea_code: i64,
    pub nivf_ref_lp_code: i64,
    pub h_type_code: i64,
    pub rn_etat_code: RnEtatCode,
    pub rn_action_code: RnActionCode,
    pub rn_voie_cote_code: RnActionCode,
    pub rn_gps_eploit_code: RnGpsEploitCode,
    pub hors_ign: String,
    pub departement_code: String,
    pub rn_cid: i64,
    pub rn_nom: String,
    pub rn_type_compl: Option<String>,
    pub insee: String,
    pub commune_nom: String,
    pub localisation: Option<String>,
    pub carte_no: String,
    pub voie_suivie: String,
    pub voie_de: Option<String>,
    pub voie_vers: Option<String>,
    pub voie_cote: RnActionCode,
    pub voie_pk: Option<String>,
    pub distance: Option<String>,
    pub rn_proche_nom: String,
    pub e: String,
    pub n: String,
    pub lambda_dms: String,
    pub phi_dms: String,
    pub support: String,
    pub support_partie: String,
    pub reper_horiz: Option<String>,
    pub reper_vertical: Option<String>,
    pub altitude: String,
    pub altitude_complementaire: String,
    pub trg_annee: String,
    pub rn_obs_date: String,
    pub rn_vis_date: String,
    pub remarque: String,
    pub triplet_cid: Option<String>,
    pub geod_info: String,
    pub canex_info: String,
    pub rn_primordial_cid: Option<i64>,
    pub sit_no: Option<String>,
    pub ptg_croquis_lettre: PtgCroquisLettre,
    pub sit_info: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum PtgCroquisLettre {
    B,
    E,
    #[serde(rename = "")]
    Empty,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum RnActionCode {
    D,
    G,
    V,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum RnEtatCode {
    E,
    N,
    P,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum RnGpsEploitCode {
    E,
    I,
    N,
    R,
}
