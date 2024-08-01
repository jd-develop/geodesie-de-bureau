mod json_mappings;
use reqwest;

#[derive(Debug, PartialEq)]
pub struct TODORenameThisStruct {
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
///    vec![TODORenameThisStruct {
///        id: 452592,
///        name: "T'.D.S3 - 50".to_string(),
///    }]
/// );
/// ```
pub fn rn_from_matricule(matricule: &str) -> Vec<TODORenameThisStruct> {
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
    let mut result_vec: Vec<TODORenameThisStruct> = vec![];
    for rn in resp.split("\n") {
        let id_and_name: Vec<&str> = rn.split("\x00").into_iter().collect();
        result_vec.push(TODORenameThisStruct {
            id: id_and_name[0].parse::<u32>().unwrap(),
            name: id_and_name[1].to_string(),
        });
    }
    result_vec
}
