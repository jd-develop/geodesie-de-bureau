pub mod json_mappings;
pub use json_mappings::{
    bbox::{BBox, Feature, Properties},
    repere::RepèreNivellement,
};
use reqwest;
use std::{
    io::{self, Write},
    process::exit,
};

const SEARCH_RN_URL: &str = "https://geodesie.ign.fr/fiches/index.php?module=e&action=visugeod";

/// This is output from the API call of searching for RNs
#[derive(Clone, Debug, PartialEq)]
pub struct RNIdentificationInfos {
    pub cid: u32,
    pub matricule: String,
}

/// Returns the names and IDs of the benchmarks that contain the provided string
///
/// # Examples
/// ```
/// use geodesie_de_bureau::ngf::*;
/// assert_eq!(
///    rn_from_matricule("T'.D.S3 - 50"),
///    vec![RNIdentificationInfos {
///        cid: 452592,
///        matricule: "T'.D.S3 - 50".to_string(),
///    }]
/// );
/// assert_eq!(
///     rn_from_matricule("PeuDeChancesQueCeSoitUnRN"),
///     vec![]
/// )
/// ```
pub fn rn_from_matricule(matricule: &str) -> Vec<RNIdentificationInfos> {
    assert_ne!(matricule, "");
    assert!(!matricule.contains("|"));
    // Let’s own the string, we have a lot to do onto it
    let mut matricule: String = String::from(matricule.trim());
    matricule = matricule.replace("’", "'");
    let mut headers: reqwest::header::HeaderMap = reqwest::header::HeaderMap::new();
    headers.insert(
        "content-type",
        "application/x-www-form-urlencoded".parse().unwrap(),
    );
    let client: reqwest::blocking::Client = reqwest::blocking::Client::new();
    let body: String =
        format!("repere_ajax={matricule}&identifiant_visugeod=identificateur_repere");
    let mut resp: String = client
        .post(SEARCH_RN_URL)
        .body(body)
        .headers(headers)
        .send()
        .unwrap()
        .error_for_status()
        .unwrap()
        .text()
        .unwrap();
    if resp.contains("Pas de résultat") {
        return vec![];
    }
    resp = resp.replace("<ul>", "");
    resp = resp.replace("</ul>", "");
    resp = resp.replace("<li id=\"", "");
    resp = resp.replace("\"><span><b>", "\x00"); // We use \x00 as a separator
    resp = resp.replace("</b></span></li>", "");
    resp = resp[..resp.find("<!--").unwrap_or(resp.len())].to_string();
    resp = resp.trim().to_string();
    let mut result_vec: Vec<RNIdentificationInfos> = vec![];
    for rn in resp.split("\n") {
        let id_and_name: Vec<&str> = rn.split("\x00").into_iter().collect();
        result_vec.push(RNIdentificationInfos {
            cid: id_and_name[0]
                .parse::<u32>()
                .unwrap_or_else(|_| panic!("Invalid CID : « {} »", id_and_name[0])),
            matricule: id_and_name[1].to_string(),
        });
    }
    result_vec
}

/// Prompts the user to select a repère in the provided repères_found
pub fn find_matricule_to_use_from_list(
    matricule_input: &str,
    repères_found: &Vec<RNIdentificationInfos>,
) -> RNIdentificationInfos {
    // If there is only one repère in the list. we return it directly
    if repères_found.len() == 1 {
        return repères_found[0].clone();
    }
    // If there are no repères in the list. we can call exit(1)
    if repères_found.len() == 0 {
        exit(1);
    }
    // If there is one repère in the list that matches exactly the provided matricule, we return it directly
    if let Some(repère) = repères_found
        .iter()
        .find(|infos_repère| infos_repère.matricule == matricule_input)
    {
        return repère.clone();
    }
    let mut string_found_repères: String = String::new();
    // Get some sizes to align the text (we therefore need to parse the list twice)
    let mut max_size_indexes: usize = 0;
    let mut max_size_matricule: usize = 0;
    let mut max_size_cid: usize = 0;
    for (index, repère) in repères_found.iter().enumerate() {
        if index.to_string().len() > max_size_indexes {
            max_size_indexes = index.to_string().len();
        };
        if repère.matricule.to_string().len() > max_size_matricule {
            max_size_matricule = repère.matricule.to_string().len();
        };
        if repère.cid.to_string().len() > max_size_cid {
            max_size_cid = repère.cid.to_string().len();
        };
    }
    for (index, repère) in repères_found.iter().enumerate() {
        string_found_repères += format!(
            "\x1b[92;1m{index:<2$}\x1b[39;22m : \x1b[94;1m{:<3$}\x1b[39;22m (id \x1b[95;1m{:>4$}\x1b[39;22m)\n",
            repère.matricule,
            repère.cid,
            max_size_indexes,
            max_size_matricule,
            max_size_cid
        )
        .as_str();
    }
    println!(
        "\
    Repères found :\n\
    {string_found_repères}\
    "
    );
    loop {
        print!(
            "Your choice (\x1b[92;1m0\x1b[22m-\x1b[1m{number_of_repères}\x1b[39;22m) : ",
            number_of_repères = repères_found.len() - 1
        );
        let mut input: String = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();
        let choice: i16 = input.parse().unwrap_or(-1);
        let repères_found_len_i16: i16 = repères_found.len() as i16;
        if (0..repères_found_len_i16).contains(&choice) {
            break repères_found[choice as usize].clone();
        };
        println!(
            "Please enter a valid choice from \x1b[92;1m0\x1b[22m to \x1b[1m{number_of_repères}\x1b[39;22m",
            number_of_repères = repères_found.len() - 1
        );
    }
}

/// Takes the identification of a RN as parameter and returns a RepèreNivellement. As simple as that !
///
/// # Examples
/// ```
/// use geodesie_de_bureau::ngf::{
///     get_rn_from_rn_identifications_infos,
///     json_mappings::bbox::*,
///     RepèreNivellement,
///     RNIdentificationInfos,
/// };
/// assert_eq!(
///     get_rn_from_rn_identifications_infos(
///         RNIdentificationInfos{
///             matricule: "T'.D.S3 - 50".to_string(),
///             cid: 452592
///         }
///     ),
///     RepèreNivellement {
///         matricule: "T'.D.S3 - 50".to_string(),
///         cid: 452592,
///         fiche_url: "https://geodesie.ign.fr/fiches/index.php?module=e&action=fichepdf&source=gp&rn_cid=452592&geo_cid=0".to_string(),
///         système_altimétrique: NivfReaCode::NgfIgn1969,
///         altitude: "125,719".to_string(),
///         altitude_complémentaire: "".to_string(),
///         altitude_type: HTypeCode::AltitudeNormale,
///         dernière_observation: "01/01/1920".to_string(),
///         nouveau_calcul: "1984".to_string(),
///         dernière_visite: "01/01/1920".to_string(),
///         état: RnÉtatCode::BonÉtat,
///         rn_type: RnTypeCode::RepèreCylindriqueDuNivellementGénéral,
///         type_complément: Some("".to_string()),
///         canex_info: "".to_string(),
///         type_complément_avec_canex: "".to_string(),
///         longitude: 1.409197,
///         latitude: 43.649276,
///         e: "571.63".to_string(),
///         n: "6284.65".to_string(),
///         département: "31".to_string(),
///         insee: "31555".to_string(),
///         commune: "Toulouse".to_string(),
///         voie_suivie: "GARONNE (LA)".to_string(),
///         voie_de: Some("LE PONT DE GRENADE".to_string()),
///         voie_vers: Some("LE PONT SAINT-MICHEL".to_string()),
///         voie_côté: VoieCôtéCode::Gauche,
///         voie_pk: Some("120,3".to_string()),
///         distance: None,
///         du_repère: "".to_string(),
///         localisation: Some("AU QUARTIER DE GINESTOUS, SUR UN RUISSEAU DE LA RIVE DROITE DE LA GARONNE".to_string()),
///         support: "PONCEAU".to_string(),
///         partie_support: Some("MUR EN RETOUR COTE FLEUVE, FACE AMONT".to_string()),
///         repèrement_horizontal: Some("A L'AXE".to_string()),
///         repèrement_vertical: Some("".to_string()),
///         hors_ign: "100063".to_string(),
///         remarques: "".to_string(),
///         exploitabilité_gps: RnGPSExploitCode::Empty,
///         géod_info: "".to_string()
///     }
/// )
/// ```
pub fn get_rn_from_rn_identifications_infos(
    rn_id_infos: RNIdentificationInfos,
) -> RepèreNivellement {
    let matricule_with_double_primes: String = rn_id_infos.matricule.replace("'", "''");
    let body: String = format!("h_recherche=repere|{matricule_with_double_primes}&t=france");
    let mut headers: reqwest::header::HeaderMap = reqwest::header::HeaderMap::new();
    headers.insert(
        "content-type",
        "application/x-www-form-urlencoded".parse().unwrap(),
    );
    let resp = reqwest::blocking::Client::new()
        .post(format!(
            "https://geodesie.ign.fr/ripgeo/fr/api/nivrn/bbox/{}/json/",
            reqwest::blocking::Client::new()
                .post(SEARCH_RN_URL)
                .body(body)
                .headers(headers)
                .send()
                .unwrap()
                .error_for_status()
                .unwrap()
                .text()
                .unwrap()
                .split("\n")
                .into_iter()
                .collect::<Vec<&str>>()[0]
                .split("|")
                .into_iter()
                .collect::<Vec<&str>>()[0]
                .split(" ")
                .into_iter()
                .map(|coord| {
                    format!(
                        "{:.1}",
                        (coord
                            .parse::<f64>()
                            .unwrap_or_else(|_| panic!("Invalid cast to f64 : « {coord} »"))
                            * 10f64)
                            .floor()
                            / 10f64
                    )
                })
                .collect::<Vec<String>>()
                .join("/")
        ))
        .send()
        .unwrap()
        .error_for_status()
        .unwrap()
        .text()
        .unwrap();
    // To debug issues related to deserialization (this deserialization fails very often)
    if let Err(wrong_col) = serde_json::from_str::<BBox>(resp.as_str()) {
        let wrong_col = wrong_col.column();
        println!(
            "{}\x1b[91;1m{}\x1b[0m{}",
            resp[wrong_col - 100..wrong_col].to_string(),
            resp[wrong_col..=wrong_col].to_string(),
            resp[wrong_col + 1..wrong_col + 100].to_string()
        );
    };
    let bbox_data: BBox = serde_json::from_str::<BBox>(resp.as_str()).unwrap();
    let rn: Feature = bbox_data
        .features
        .iter()
        .find(|feature| feature.properties.rn_nom == rn_id_infos.matricule)
        .unwrap()
        .clone();
    let prop: Properties = rn.properties;
    // type_complément_avec_canex
    let type_complement = prop.rn_type_compl.clone().unwrap_or("".to_string());
    let canex_info = prop.canex_info.clone();
    let type_complément_avec_canex: String;
    if type_complement.is_empty() && canex_info.is_empty() {
        type_complément_avec_canex = "".to_string()
    } else if type_complement.is_empty() {
        type_complément_avec_canex = canex_info
    } else if canex_info.is_empty() {
        type_complément_avec_canex = type_complement
    } else {
        type_complément_avec_canex = type_complement.to_string() + ", " + canex_info.as_str()
    }
    RepèreNivellement {
        matricule: prop.rn_nom,
        cid: prop.rn_cid,
        fiche_url: format!("https://geodesie.ign.fr/fiches/index.php?module=e&action=fichepdf&source=gp&rn_cid={cid}&geo_cid=0", cid= prop.rn_cid),
        système_altimétrique: prop.nivf_rea_code,
        altitude: prop.altitude,
        altitude_complémentaire: prop.altitude_complementaire,
        altitude_type: prop.h_type_code,
        dernière_observation: prop.rn_obs_date,
        nouveau_calcul: prop.trg_annee,
        dernière_visite: prop.rn_vis_date,
        état: prop.rn_etat_code,
        rn_type: prop.rn_type_code,
        type_complément: prop.rn_type_compl,
        canex_info: prop.canex_info,
        type_complément_avec_canex,
        longitude: rn.geometry.coordinates[0],
        latitude: rn.geometry.coordinates[1],
        e: prop.e,
        n: prop.n,
        département: prop.departement_code,
        insee: prop.insee,
        commune: prop.commune_nom,
        voie_suivie: prop.voie_suivie,
        voie_de: prop.voie_de,
        voie_vers: prop.voie_vers,
        voie_côté: prop.voie_cote,
        voie_pk: prop.voie_pk,
        distance: prop.distance,
        du_repère: prop.rn_proche_nom,
        localisation: prop.localisation,
        support: prop.support,
        partie_support: prop.support_partie,
        repèrement_horizontal: prop.reper_horiz,
        repèrement_vertical: prop.reper_vertical,
        hors_ign: prop.hors_ign,
        remarques: prop.remarque,
        exploitabilité_gps: prop.rn_gps_eploit_code,
        géod_info: prop.geod_info
    }
}

#[test]
fn tests_rn_from_matricule() {
    for repère in [
        "M.AC - 0-VIII",
        "N.P.K3Q3 - 56",
        "N.P.K3Q3 - 57",
        "T'.D.S3 - 102a",
        "M\".A.K3L3 - 15-I",
        "FM\" - 3-VIII",
    ] {
        assert_eq!(
            rn_from_matricule(repère),
            vec![RNIdentificationInfos {
                cid: match repère {
                    "M.AC - 0-VIII" => 303869,
                    "N.P.K3Q3 - 56" => 266242,
                    "N.P.K3Q3 - 57" => 364934,
                    "T'.D.S3 - 102a" => 481679,
                    "M\".A.K3L3 - 15-I" => 540629,
                    "FM\" - 3-VIII" => 540745,
                    _ => panic!(
                        "The provided repère name has no associated value in the match expression"
                    ),
                },
                matricule: repère.to_string(),
            }]
        );
    }
    assert_eq!(
        rn_from_matricule("T'.D.S3 - 5"),
        vec![
            RNIdentificationInfos {
                cid: 452592,
                matricule: "T'.D.S3 - 50".to_string(),
            },
            RNIdentificationInfos {
                cid: 429495,
                matricule: "T'.D.S3 - 52".to_string(),
            },
            RNIdentificationInfos {
                cid: 108049,
                matricule: "T'.D.S3 - 54".to_string(),
            },
            RNIdentificationInfos {
                cid: 108050,
                matricule: "T'.D.S3 - 55".to_string(),
            },
            RNIdentificationInfos {
                cid: 452593,
                matricule: "T'.D.S3 - 56".to_string(),
            },
            RNIdentificationInfos {
                cid: 338593,
                matricule: "T'.D.S3 - 57 BIS".to_string(),
            },
            RNIdentificationInfos {
                cid: 429496,
                matricule: "T'.D.S3 - 58".to_string(),
            },
            RNIdentificationInfos {
                cid: 521727,
                matricule: "T'.D.S3 - 59".to_string(),
            },
            RNIdentificationInfos {
                cid: 481574,
                matricule: "T'.D.S3 - 5 BIS".to_string(),
            },
        ]
    );
}
#[test]
fn test_get_rn_from_rn_identifications_infos() {
    assert_eq!(get_rn_from_rn_identifications_infos(
        RNIdentificationInfos {
            cid: 452592,
            matricule: "T'.D.S3 - 50".to_string(),
        }
    ), RepèreNivellement {
        matricule: "T'.D.S3 - 50".to_string(),
        cid: 452592,
        fiche_url: "https://geodesie.ign.fr/fiches/index.php?module=e&action=fichepdf&source=gp&rn_cid=452592&geo_cid=0".to_string(),
        système_altimétrique: json_mappings::bbox::NivfReaCode::NgfIgn1969,
        altitude: "125,719".to_string(),
        altitude_complémentaire: "".to_string(),
        altitude_type: json_mappings::bbox::HTypeCode::AltitudeNormale,
        dernière_observation: "01/01/1920".to_string(),
        nouveau_calcul: "1984".to_string(),
        dernière_visite: "01/01/1920".to_string(),
        état: json_mappings::bbox::RnÉtatCode::BonÉtat,
        rn_type: json_mappings::bbox::RnTypeCode::RepèreCylindriqueDuNivellementGénéral,
        type_complément: Some("".to_string()),
        canex_info: "".to_string(),
        type_complément_avec_canex: "".to_string(),
        longitude: 1.409197,
        latitude: 43.649276,
        e: "571.63".to_string(),
        n: "6284.65".to_string(),
        département: "31".to_string(),
        insee: "31555".to_string(),
        commune: "Toulouse".to_string(),
        voie_suivie: "GARONNE (LA)".to_string(),
        voie_de: Some("LE PONT DE GRENADE".to_string()),
        voie_vers: Some("LE PONT SAINT-MICHEL".to_string()),
        voie_côté: json_mappings::bbox::VoieCôtéCode::Gauche,
        voie_pk: Some("120,3".to_string()),
        distance: None,
        du_repère: "".to_string(),
        localisation: Some("AU QUARTIER DE GINESTOUS, SUR UN RUISSEAU DE LA RIVE DROITE DE LA GARONNE".to_string()),
        support: "PONCEAU".to_string(),
        partie_support: Some("MUR EN RETOUR COTE FLEUVE, FACE AMONT".to_string()),
        repèrement_horizontal: Some("A L'AXE".to_string()),
        repèrement_vertical: Some("".to_string()),
        hors_ign: "100063".to_string(),
        remarques: "".to_string(),
        exploitabilité_gps: json_mappings::bbox::RnGPSExploitCode::Empty,
        géod_info: "".to_string()
    })
}
