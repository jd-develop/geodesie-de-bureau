mod json_mappings;
use reqwest;

#[derive(Debug, PartialEq)]
pub struct RNIdentificationInfos {
    pub id: u32,
    pub name: String,
}

/// Returns the names and IDs of the benchmarks that contain the provided string
///
/// # Examples
/// ```
/// use geodesie_de_bureau::*;
/// assert_eq!(
///    rn_from_matricule("T'.D.S3 - 50"),
///    vec![RNIdentificationInfos {
///        id: 452592,
///        name: "T'.D.S3 - 50".to_string(),
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
        .post("https://geodesie.ign.fr/fiches/index.php?module=e&action=visugeod")
        .body(body)
        .headers(headers)
        .send()
        .unwrap()
        .text()
        .unwrap();
    dbg!(&resp);
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
            id: id_and_name[0].parse::<u32>().unwrap(),
            name: id_and_name[1].to_string(),
        });
    }
    result_vec
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
                id: match repère {
                    "M.AC - 0-VIII" => 303869,
                    "N.P.K3Q3 - 56" => 266242,
                    "N.P.K3Q3 - 57" => 364934,
                    "T'.D.S3 - 102a" => 481679,
                    "M\".A.K3L3 - 15-I" => 540629,
                    "FM\" - 3-VIII" => 540745,
                    _ => panic!("The provided repère name has no associated value in the match expression"),
                },
                name: repère.to_string(),
            }]
        );
    }
    assert_eq!(
        rn_from_matricule("T'.D.S3 - 5"),
        vec![
            RNIdentificationInfos {
                id: 452592,
                name: "T'.D.S3 - 50".to_string(),
            },
            RNIdentificationInfos {
                id: 429495,
                name: "T'.D.S3 - 52".to_string(),
            },
            RNIdentificationInfos {
                id: 108049,
                name: "T'.D.S3 - 54".to_string(),
            },
            RNIdentificationInfos {
                id: 108050,
                name: "T'.D.S3 - 55".to_string(),
            },
            RNIdentificationInfos {
                id: 452593,
                name: "T'.D.S3 - 56".to_string(),
            },
            RNIdentificationInfos {
                id: 338593,
                name: "T'.D.S3 - 57 BIS".to_string(),
            },
            RNIdentificationInfos {
                id: 429496,
                name: "T'.D.S3 - 58".to_string(),
            },
            RNIdentificationInfos {
                id: 521727,
                name: "T'.D.S3 - 59".to_string(),
            },
            RNIdentificationInfos {
                id: 481574,
                name: "T'.D.S3 - 5 BIS".to_string(),
            },
        ]
    );
}
