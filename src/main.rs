use geodesie_de_bureau::*;

fn main() {
    dbg!(find_matricule_to_use_from_list(
        "T'.D.S3 - 5",
        &rn_from_matricule("T'.D.S3 - 5")
    ));
}
