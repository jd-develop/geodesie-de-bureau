use geodesie_de_bureau::*;

fn main() {
    dbg!(get_rn_from_rn_identifications_infos(
        find_matricule_to_use_from_list("T'.D.S3 - 50", &rn_from_matricule("T'.D.S3 - 50"))
    ));
}
