use geodesie_de_bureau::*;

fn main() {
    dbg!(ngf::get_rn_from_rn_identifications_infos(
        ngf::find_matricule_to_use_from_list(
            "T'.D.S3 - 50",
            &ngf::rn_from_matricule("T'.D.S3 - 50")
        )
    ));
}
