mod json_mappings;
use json_mappings::repere::RepèreNivellement;
use std::collections::HashMap;
use reqwest;

/// Returns a RepèreNivellement object
fn rn_from_matricule(matricule: &str) -> RepèreNivellement {
    assert_ne!(matricule, "");
    // Let’s own the string, we have a lot to do onto it
    let mut matricule: String = String::from(matricule.trim());
    matricule = matricule.replace("’", "'");
    matricule = matricule.replace("'", "''");
    let matricule_upper_candidate1: String = matricule.as_str()[..matricule.len() - 1]
        .to_uppercase()
        + &matricule.as_str()[matricule.len()..matricule.len() + 1];
    let matricule_upper_candidate2: String = matricule.to_uppercase();
    let url: &str = "https://geodesie.ign.fr/fiches/index.php?module=e&action=visugeod";
    let client = reqwest::blocking::Client::new();
    let mut data = HashMap::new();
    data.insert("h_recherche", "");
    todo!();
    let mut headers = reqwest::header::HeaderMap::new();
    let header_name: reqwest::header::HeaderName = "content-type".parse().unwrap();
    headers.insert(header_name, "application/x-www-form-urlencoded".parse().unwrap());
    let resp: reqwest::blocking::Response = client.post(url).headers(headers).json(&data).send().unwrap();
    
    RepèreNivellement {}
}
