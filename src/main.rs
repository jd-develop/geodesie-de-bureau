use geodesie_de_bureau::*;
use ngf::*;

fn main() {
    let rn: RepèreNivellement =
        ngf::get_rn_from_rn_identifications_infos(ngf::find_matricule_to_use_from_list(
            "T'.D.S3 - 50",
            &ngf::rn_from_matricule("T'.D.S3 - 50"),
        ));
    println!("{}, {}, {}", rn.commune, rn.matricule, rn.fiche_url)
}
