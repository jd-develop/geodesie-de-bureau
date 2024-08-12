use clap::{arg, command, value_parser};
use geodesie_de_bureau::*;
use ngf::*;

fn main() {
    // let _ = cli_interface::cli_interface::main();

    let matches: clap::ArgMatches = command!()
        .arg(
            arg!(
                -m --matricule <matricule> "Matricule du repère à visualiser"
            )
            .required(true)
            .value_parser(value_parser!(String)),
        )
        .get_matches();
    if let Some(rn_matricule) = matches.get_one::<String>("matricule") {
        let rn: RepèreNivellement =
            ngf::get_rn_from_rn_identifications_infos(ngf::find_matricule_to_use_from_list(
                rn_matricule,
                &ngf::rn_from_matricule(rn_matricule),
            ));
        println!("{}", rn);
    } else {
        println!("Please specify a matricule")
    }
}
