use clap::{arg, command, value_parser};
use geodesie_de_bureau::*;
use ngf::*;
use ratatui::{
    crossterm::{
        event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    layout::{Alignment, Rect},
    prelude::*,
    symbols::border,
    widgets::{
        block::{Position, Title},
        Block, Paragraph,
    },
};
use std::io::{stdout, Result};

fn main() -> Result<()> {
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
    let rn: Option<RepèreNivellement>;
    if let Some(rn_matricule) = matches.get_one::<String>("matricule") {
        rn = Some(ngf::get_rn_from_rn_identifications_infos(
            ngf::find_matricule_to_use_from_list(
                rn_matricule,
                &ngf::rn_from_matricule(rn_matricule),
            ),
        ));
    } else {
        rn = None;
    }

    let mut terminal = cli::startup::init()?;
    let app_result = cli::startup::App::default().run(&mut terminal);
    cli::startup::restore()?;
    app_result
}
