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
    pub rn_type_code: RnTypeCode,
    pub nivf_ref_en_code: i64,
    pub nivf_rea_code: NivfReaCode,
    pub nivf_ref_lp_code: i64,
    pub h_type_code: i64,
    pub rn_etat_code: RnÉtatCode,
    pub rn_action_code: RnActionCode,
    pub rn_voie_cote_code: VoieCôtéCode,
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
    pub voie_cote: VoieCôtéCode,
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
pub enum RnÉtatCode {
    #[serde(rename = "D")]
    Détruit,
    #[serde(rename = "E")]
    BonÉtat,
    #[serde(rename = "I")]
    Imprenable,
    #[serde(rename = "M")]
    MauvaisÉtat,
    #[serde(rename = "N")]
    NonRetrouvé,
    #[serde(rename = "P")]
    PresuméDéplacé,
    #[serde(rename = "Y")]
    DétruitAprèsObservation,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum RnActionCode {
    #[serde(rename = "D")]
    Détermination,
    #[serde(rename = "V")]
    Visite,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum VoieCôtéCode {
    #[serde(rename = "D")]
    Droit,
    #[serde(rename = "G")]
    Gauche,
    #[serde(rename = "M")]
    Milieu,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum RnGpsEploitCode {
    #[serde(rename = "E")]
    ExploitableDirectementParGPS,
    #[serde(rename = "I")]
    InexploitableParGPS,
    #[serde(rename = "R")]
    ExploitableParGPSDepuisUneStationExcentrée,
    #[serde(rename = "N")]
    Empty,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum RnTypeCode {
    #[serde(rename = "000")]
    Inconnu,
    #[serde(rename = "001")]
    RepèreConsole,
    #[serde(rename = "007")]
    Rivet,
    #[serde(rename = "008")]
    RepèreBourdalouë,
    #[serde(rename = "009")]
    RepèrePLMCheminDeFerParisLyonMéditerranée,
    #[serde(rename = "010")]
    RepèreMRUMinistèreReconstructionUrbanisme,
    #[serde(rename = "011")]
    RepèrePontsEtChaussées,
    #[serde(rename = "012")]
    RepèreNavigation,
    #[serde(rename = "013")]
    RepèreVilleDeParis,
    #[serde(rename = "014")]
    RepèreCylindriqueDuNivellementGénéral,
    #[serde(rename = "015")]
    RepèreLocal,
    #[serde(rename = "016")]
    RepèreHexagonal,
    #[serde(rename = "017")]
    RepèreLocalRepèreDansUnSystèmeLocal,
    #[serde(rename = "018")]
    ÉchelleHydrométrique,
    #[serde(rename = "019")]
    RepèreBoule,
    #[serde(rename = "020")]
    RepèreItalien,
    #[serde(rename = "021")]
    RepèreDeCrue,
    #[serde(rename = "022")]
    RepèreOctogonal,
    #[serde(rename = "023")]
    RepèreReconstruction,
    #[serde(rename = "024")]
    RepèreEDF,
    #[serde(rename = "025")]
    RepèreSNCF,
    #[serde(rename = "026")]
    RepèreCadastre,
    #[serde(rename = "027")]
    RepèreAllemand,
    #[serde(rename = "028")]
    RepèreBelge,
    #[serde(rename = "029")]
    RepèreLuxembourgeois,
    #[serde(rename = "030")]
    RepèreSuisse,
    #[serde(rename = "031")]
    RepèreEspagnol,
    #[serde(rename = "032")]
    RepèreVilleDeMarseille,
    #[serde(rename = "033")]
    TraitDeCrue,
    #[serde(rename = "034")]
    Borne,
    #[serde(rename = "035")]
    RepèreSHOMServiceHydrographiqueEtOcéanographiqueDeLaMarine,
    #[serde(rename = "036")]
    RepèreFondamental,
    #[serde(rename = "037")]
    Tube,
    #[serde(rename = "038")]
    RepèreIPGInstitutDePhysiqueDuGlobe,
    #[serde(rename = "039")]
    RepèreConique,
    #[serde(rename = "040")]
    RepèreEnFonteTriangulaire,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum NivfReaCode {
    #[serde(rename = "2")]
    NgfIgn1969 = 1969,
    #[serde(rename = "3")]
    NgfIgn1978 = 1978,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum PtgCroquisLettre {
    B,
    E,
    #[serde(rename = "")]
    Empty,
}




