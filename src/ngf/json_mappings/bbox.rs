//! Can serialize / deserialize the output of the « https://geodesie.ign.fr/ripgeo/fr/api/nivrn/bbox/{long}/{lat}/json/ » API call
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct BBox {
    #[serde(rename = "type")]
    pub bbox_type: String,
    pub features: Vec<Feature>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Feature {
    #[serde(rename = "type")]
    pub feature_type: FeatureType,
    pub geometry: Geometry,
    pub properties: Properties,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum FeatureType {
    Feature,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Geometry {
    #[serde(rename = "type")]
    pub geometry_type: GeometryType,
    pub coordinates: Vec<f64>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum GeometryType {
    Point,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Properties {
    pub image_name: String,
    pub rn_type_code: RnTypeCode,
    pub nivf_ref_en_code: NivfRefEnCode,
    pub nivf_rea_code: NivfReaCode,
    pub nivf_ref_lp_code: i64,
    pub h_type_code: HTypeCode,
    pub rn_etat_code: RnÉtatCode,
    pub rn_action_code: RnActionCode,
    pub rn_voie_cote_code: VoieCôtéCode,
    pub rn_gps_eploit_code: RnGPSExploitCode,
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
    pub support_partie: Option<String>,
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
    pub ptg_croquis_lettre: String,
    pub sit_info: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
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

impl fmt::Display for RnTypeCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RnTypeCode::Inconnu => "Inconnu",
                RnTypeCode::RepèreConsole => "Repère console",
                RnTypeCode::Rivet => "Rivet",
                RnTypeCode::RepèreBourdalouë => "Repère Bourdalouë",
                RnTypeCode::RepèrePLMCheminDeFerParisLyonMéditerranée =>
                    "Repère PLM (Chemin de fer Paris Lyon Méditerranée)",
                RnTypeCode::RepèreMRUMinistèreReconstructionUrbanisme =>
                    "Repère MRU (Ministère Reconstruction Urbanisme)",
                RnTypeCode::RepèrePontsEtChaussées => "Repèse ponts et chaussées",
                RnTypeCode::RepèreNavigation => "Repère navigation",
                RnTypeCode::RepèreVilleDeParis => "Repèse ville de Paris",
                RnTypeCode::RepèreCylindriqueDuNivellementGénéral =>
                    "Repère cylindrique du Nivellement Général",
                RnTypeCode::RepèreLocal => "Repère local",
                RnTypeCode::RepèreHexagonal => "Repère hexagonal",
                RnTypeCode::RepèreLocalRepèreDansUnSystèmeLocal =>
                    "Repère local, repère dans un système local",
                RnTypeCode::ÉchelleHydrométrique => "Échelle hydrométrique",
                RnTypeCode::RepèreBoule => "Repère boule",
                RnTypeCode::RepèreItalien => "Repère italien",
                RnTypeCode::RepèreDeCrue => "Repère de crue",
                RnTypeCode::RepèreOctogonal => "Repère octogonal",
                RnTypeCode::RepèreReconstruction => "Repère reconstruction",
                RnTypeCode::RepèreEDF => "Repère EDF",
                RnTypeCode::RepèreSNCF => "Repère SNCF",
                RnTypeCode::RepèreCadastre => "Repère cadastre",
                RnTypeCode::RepèreAllemand => "Repère allemand",
                RnTypeCode::RepèreBelge => "Repère belge",
                RnTypeCode::RepèreLuxembourgeois => "Repère luxembourgeois",
                RnTypeCode::RepèreSuisse => "Repère suisse",
                RnTypeCode::RepèreEspagnol => "Repère espagnol",
                RnTypeCode::RepèreVilleDeMarseille => "Repère ville de Marseille",
                RnTypeCode::TraitDeCrue => "Trait de crue",
                RnTypeCode::Borne => "Borne",
                RnTypeCode::RepèreSHOMServiceHydrographiqueEtOcéanographiqueDeLaMarine =>
                    "Repère SHOM (Service Hydrographique et Océanographique de la Marine)",
                RnTypeCode::RepèreFondamental => "Repère fondamental",
                RnTypeCode::Tube => "Tube",
                RnTypeCode::RepèreIPGInstitutDePhysiqueDuGlobe =>
                    "Repère IPG (Institut de Physique du Globe)",
                RnTypeCode::RepèreConique => "Repère conique",
                RnTypeCode::RepèreEnFonteTriangulaire => "Repère en fonte triangulaire",
            }
        )
    }
}

#[derive(Serialize_repr, Deserialize_repr, Clone, PartialEq, Debug)]
#[repr(u64)]
pub enum NivfRefEnCode {
    SystèmeRGF93v1ETRS89ProjectionLAMBERT93 = 702400037010140,
}

impl fmt::Display for NivfRefEnCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Système : RGF93 v1 (ETRS89) - Projection : LAMBERT-93")
    }
}

#[derive(Serialize_repr, Deserialize_repr, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum NivfReaCode {
    NgfIgn1969 = 2,
    NgfIgn1978 = 3,
}

impl fmt::Display for NivfReaCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                NivfReaCode::NgfIgn1969 => "NGF-IGN 1969",
                NivfReaCode::NgfIgn1978 => "NGF-IGN 1978",
            }
        )
    }
}

#[derive(Serialize_repr, Deserialize_repr, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum HTypeCode {
    AltitudeNormale = 2 | 3,
    AltitudeOrthométrique =
        10 | 11 | 13 | 14 | 15 | 16 | 17 | 18 | 21 | 23 | 26 | 29 | 35 | 37 | 41 | 44,
    AltitudeProvisoire = 169,
}

impl fmt::Display for HTypeCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                HTypeCode::AltitudeNormale => "Altitude normale",
                HTypeCode::AltitudeOrthométrique => "Altitude orthométrique",
                HTypeCode::AltitudeProvisoire => "Altitude provisoire",
            }
        )
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
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

impl fmt::Display for RnÉtatCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RnÉtatCode::Détruit => "Détruit",
                RnÉtatCode::BonÉtat => "Bon état",
                RnÉtatCode::Imprenable => "Imprenable",
                RnÉtatCode::MauvaisÉtat => "Mauvais état",
                RnÉtatCode::NonRetrouvé => "Non retrouvé",
                RnÉtatCode::PresuméDéplacé => "Présumé déplacé",
                RnÉtatCode::DétruitAprèsObservation => "Détruit après observation",
            }
        )
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum RnActionCode {
    #[serde(rename = "D")]
    Détermination,
    #[serde(rename = "V")]
    Visite,
}

impl fmt::Display for RnActionCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RnActionCode::Visite => "Visite",
                RnActionCode::Détermination => "Détermination",
            }
        )
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum VoieCôtéCode {
    #[serde(rename = "D")]
    Droit,
    #[serde(rename = "G")]
    Gauche,
    #[serde(rename = "M")]
    Milieu,
    #[serde(rename = "V")]
    TheAPIDocumentationIsWrong,
}

impl fmt::Display for VoieCôtéCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                VoieCôtéCode::Droit => "Droit",
                VoieCôtéCode::Gauche => "Gauche",
                VoieCôtéCode::Milieu => "Milieu",
                VoieCôtéCode::TheAPIDocumentationIsWrong => "The API documentation is wrong!",
            }
        )
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum RnGPSExploitCode {
    #[serde(rename = "E")]
    ExploitableDirectementParGPS,
    #[serde(rename = "I")]
    InexploitableParGPS,
    #[serde(rename = "R")]
    ExploitableParGPSDepuisUneStationExcentrée,
    #[serde(rename = "N")]
    Empty,
}

impl fmt::Display for RnGPSExploitCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RnGPSExploitCode::ExploitableDirectementParGPS => "Exploitable directement par GPS",
                RnGPSExploitCode::InexploitableParGPS => "Inexploitable par GPS",
                RnGPSExploitCode::ExploitableParGPSDepuisUneStationExcentrée =>
                    "Exploitable par GPS depuis une station excentrée",
                _ => "",
            }
        )
    }
}

// not documented
// #[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
// #[serde(rename_all = "snake_case")]
// pub enum PtgCroquisLettre {
//    B,
//    E,
//    #[serde(rename = "")]
//    Empty,
// }
