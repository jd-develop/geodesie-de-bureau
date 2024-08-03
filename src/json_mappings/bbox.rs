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
    pub rn_type_compl: Option<RnTypeCompl>,
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
    pub triplet_cid: Option<TripletCid>,
    pub geod_info: GeodInfo,
    pub canex_info: String,
    pub rn_primordial_cid: Option<i64>,
    pub sit_no: Option<String>,
    pub ptg_croquis_lettre: PtgCroquisLettre,
    pub sit_info: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum GeodInfo {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "POINT b DU SITE GEODESIQUE 3155504")]
    PointBDuSiteGeodesique3155504,
    #[serde(rename = "POINT e DU SITE GEODESIQUE 3156101")]
    PointEDuSiteGeodesique3156101,
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

#[derive(Serialize, Deserialize, Clone)]
pub enum RnTypeCompl {
    #[serde(rename = "BASSIN DE LA GARONNE")]
    BassinDeLaGaronne,
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "RECONSTRUCTION")]
    Reconstruction,
    #[serde(rename = "TRAIT NIVELE 0.20 M")]
    RnTypeComplTraitNivele020M,
    #[serde(rename = "TRAIT NIVELE: -0.05 M")]
    TraitNivele005M,
    #[serde(rename = "TRAIT NIVELE: 0.20 M")]
    TraitNivele020M,
    #[serde(rename = "TRAIT NIVELE 4.00 M")]
    TraitNivele400M,
    #[serde(rename = "TRAIT NIVELE 7.00 M")]
    TraitNivele700M,
    #[serde(rename = "VILLE DE PERIGUEUX")]
    VilleDePerigueux,
    #[serde(rename = "VILLE DE TOULOUSE")]
    VilleDeToulouse,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum TripletCid {
    #[serde(rename = "f-11616")]
    F11616,
    #[serde(rename = "f-11618")]
    F11618,
    #[serde(rename = "f-11634")]
    F11634,
}
