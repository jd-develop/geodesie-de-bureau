mod json_mappings;
use json_mappings::{
    bbox::{BBox, Feature, Properties},
    repere::RepèreNivellement,
};
use reqwest;
use std::io::{self, Write};

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
/// use geodesie_de_bureau::*;
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
            cid: id_and_name[0].parse::<u32>().unwrap(),
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
///
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
    let bbox_data: BBox = serde_json::from_str::<BBox>(
        reqwest::blocking::Client::new()
            .post(format!(
                "https://geodesie.ign.fr/ripgeo/fr/api/nivrn/bbox/{}/json/",
                reqwest::blocking::Client::new()
                    .post(SEARCH_RN_URL)
                    .body(body)
                    .headers(headers)
                    .send()
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
                            (coord.parse::<f64>().unwrap() * 10f64).floor() / 10f64
                        )
                    })
                    .collect::<Vec<String>>()
                    .join("/")
            ))
            .send()
            .unwrap()
            .text()
            .unwrap()
            .as_str(),
    )
    .unwrap();
    let rn: Feature = bbox_data
        .features
        .iter()
        .find(|feature| feature.properties.rn_nom == rn_id_infos.matricule)
        .unwrap()
        .clone();
    // TODO Map the Feature struct fields to the RepèreNivellement struct fields
    RepèreNivellement {}
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
