use std::fmt::Display;

use super::bbox::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct RepèreNivellement {
    pub matricule: String,
    pub cid: i64,
    pub fiche_url: String,
    pub système_altimétrique: NivfReaCode,
    pub altitude: String,
    pub altitude_complémentaire: String,
    pub altitude_type: HTypeCode,

    pub dernière_observation: String,
    pub nouveau_calcul: String,
    pub dernière_visite: String,
    pub état: RnÉtatCode,

    pub rn_type: RnTypeCode,
    pub type_complément: Option<String>,
    pub canex_info: String,
    pub type_complément_avec_canex: String,

    pub longitude: f64,
    pub latitude: f64,
    pub e: String,
    pub n: String,

    pub département: String,
    pub insee: String,
    pub commune: String,
    pub voie_suivie: String,
    pub voie_de: Option<String>,
    pub voie_vers: Option<String>,
    pub voie_côté: VoieCôtéCode,
    pub voie_pk: Option<String>,
    pub distance: Option<String>,
    pub du_repère: String,
    pub localisation: Option<String>,

    pub support: String,
    pub partie_support: Option<String>,
    pub repèrement_horizontal: Option<String>,
    pub repèrement_vertical: Option<String>,

    pub hors_ign: String,
    pub remarques: String,
    pub exploitabilité_gps: RnGPSExploitCode,
    pub géod_info: String,
}

impl Display for RepèreNivellement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "\
            \x1b[91m=============== Repère de nivellement ===============\x1b[39m\n\
            \x1b[94mFiche en ligne\x1b[39m : {fiche_url}\n\
            \n\
            \x1b[94mMatricule\x1b[39m : {matricule}\n\
            \x1b[94mSystème altimétrique\x1b[39m : {système_altimétrique:?}\n\
            \x1b[94mAltitude\x1b[39m : {altitude}m ({altitude_type:?})\n\
            {altitude_complémentaire}\
            \n\
            \x1b[91m=== Dernière visite et observation ===\x1b[39m\n\
            \x1b[94mAnnée de dernière observation\x1b[39m : {dernière_observation}\n\
            \x1b[94mAnnée de nouveau calcul\x1b[39m : {nouveau_calcul}\n\
            \x1b[94mDernière visite\x1b[39m : {dernière_visite}\n\
            \x1b[94mÉtat\x1b[39m : {état}\n\
            \n\
            \x1b[91m=== Type ===\x1b[39m\n\
            \x1b[94mType\x1b[39m : {rn_type:?}\n\
            {type_complément_avec_canex}\
            \n\
            \x1b[91m=== Coordonnées DMS ===\x1b[39m\n\
            \x1b[94mLongitude (dms)\x1b[39m : {longitude}\n\
            \x1b[94mLatitude (dms)\x1b[39m : {latitude}\n\
            \n\
            \x1b[91m=== Coordonnées en mètres ===\x1b[39m\n\
            \x1b[94mE (m)\x1b[39m : {e}\n\
            \x1b[94mN (m)\x1b[39m : {n}\n\
            \n\
            \x1b[91m=== Localisation ===\x1b[39m\n\
            \x1b[94mDépartement\x1b[39m : {département}\n\
            \x1b[94mNuméro insee\x1b[39m : {insee}\n\
            \x1b[94mCommune\x1b[39m : {commune}\n\
            \x1b[94mVoie suivie\x1b[39m : {voie_suivie}\n\
            {voie_de}\
            {voie_vers}\
            {voie_côté}\
            {voie_pk}\
            {distance}\
            {localisation}\
            \n\
            \x1b[91m=== Support ===\x1b[39m\n\
            \x1b[94mSupport\x1b[39m : {support}{géod_info}\n\
            {partie_support}\
            \x1b[94mRepèrements\x1b[39m :\n\
            {repèrement_horizontal}\
            {repèrement_vertical}\
            {remarques}\
            {hors_ign}\
            ",
            fiche_url = self.fiche_url,
            matricule = self.matricule,
            système_altimétrique = self.système_altimétrique,
            altitude = self.altitude,
            altitude_type = self.altitude_type,
            altitude_complémentaire = if self.altitude_complémentaire.is_empty() {
                "".to_string()
            } else {
                format!("\x1b[94mAltitude\x1b[39m : {altitude_complémentaire}m (Altitude complémentaire)\n", altitude_complémentaire = self.altitude_complémentaire)
            },
            dernière_observation = self.dernière_observation,
            nouveau_calcul = self.nouveau_calcul,
            dernière_visite = self.dernière_visite,
            état = match self.état {
                RnÉtatCode::Détruit => "\x1b[91mDétruit\x1b[39m",
                RnÉtatCode::BonÉtat => "\x1b[92mBon état\x1b[39m",
                RnÉtatCode::Imprenable => "\x1b[94mImprenable\x1b[39m",
                RnÉtatCode::MauvaisÉtat => "\x1b[93Mauvais état\x1b[39m",
                RnÉtatCode::NonRetrouvé => "\x1b[90Non retrouvé\x1b[39m",
                RnÉtatCode::PresuméDéplacé => "\x1b[96mPrésumé déplacé\x1b[39m",
                RnÉtatCode::DétruitAprèsObservation =>
                    "\x1b[95mDétruit après observation\x1b[39m",
            },
            rn_type = self.rn_type,
            type_complément_avec_canex = if self.type_complément_avec_canex.is_empty() {
                "".to_string()
            } else {
                format!(
                    "\x1b[94mComplément\x1b[39m : {type_complément_avec_canex}\n",
                    type_complément_avec_canex = self.type_complément_avec_canex
                )
            },
            longitude = self.longitude,
            latitude = self.latitude,
            e = self.e,
            n = self.n,
            département = self.département,
            insee = self.insee,
            commune = self.commune,
            voie_suivie = self.voie_suivie,
            voie_de = if let Some(voie_de) = &self.voie_de {
                format!(
                    "{first_chars} \x1b[94mde\x1b[39m : {voie_de}\n",
                    first_chars = if self.voie_vers.is_some()
                        || self.voie_côté != VoieCôtéCode::TheAPIDocumentationIsWrong
                        || self.voie_pk.is_some()
                    {
                        "├╴"
                    } else {
                        "└╴"
                    }
                )
            } else {
                "".to_string()
            },
            voie_vers = if let Some(voie_vers) = &self.voie_vers {
                format!(
                    "{first_chars} \x1b[94mà\x1b[39m : {voie_vers}\n",
                    first_chars = if self.voie_côté != VoieCôtéCode::TheAPIDocumentationIsWrong
                        || self.voie_pk.is_some()
                    {
                        "├╴"
                    } else {
                        "└╴"
                    }
                )
            } else {
                "".to_string()
            },
            voie_côté = if self.voie_côté != VoieCôtéCode::TheAPIDocumentationIsWrong {
                format!(
                    "{first_chars} \x1b[94mcôté\x1b[39m : {voie_côté:?}\n",
                    first_chars = if self.voie_pk.is_some() {
                        "├╴"
                    } else {
                        "└╴"
                    },
                    voie_côté = self.voie_côté
                )
            } else {
                "".to_string()
            },
            voie_pk = if let Some(voie_pk) = &self.voie_pk {
                format!("└╴ \x1b[94mPK\x1b[39m : {voie_pk}\n")
            } else {
                "".to_string()
            },
            distance = if let Some(distance) = &self.distance {
                format!(
                    "\
                    \x1b[94mDistance\x1b[39m : {distance}km\n\
                    └╴ \x1b[94mdu repère\x1b[39m : {du_repère}\n\
                    ",
                    du_repère = self.du_repère
                )
            } else {
                "".to_string()
            },
            localisation = if let Some(localisation) = &self.localisation {
                format!("\x1b[94mLocalisation\x1b[39m : {localisation}\n")
            } else {
                "".to_string()
            },
            support = self.support,
            géod_info = if self.géod_info.is_empty() {
                "".to_string()
            } else {
                format!(" ({géod_info})", géod_info = self.géod_info)
            },
            partie_support = if let Some(partie_support) = &self.partie_support {
                format!("\x1b[94mPartie du support\x1b[39m : {partie_support}\n")
            } else {
                "".to_string()
            },
            repèrement_horizontal = if let Some(repèrement_horizontal) = &self.repèrement_horizontal
            {
                if repèrement_horizontal != "" {
                    format!(
                        "{first_chars} \x1b[94mhorizontal\x1b[39m : {repèrement_horizontal}\n",
                        first_chars = if let Some(repèrement_vertical) = &self.repèrement_vertical
                        {
                            if repèrement_vertical != "" {
                                "├╴"
                            } else {
                                "└╴"
                            }
                        } else {
                            "└╴"
                        }
                    )
                } else {
                    "".to_string()
                }
            } else {
                "".to_string()
            },
            repèrement_vertical = if let Some(repèrement_vertical) = &self.repèrement_vertical {
                if repèrement_vertical != "" {
                    format!("└╴ \x1b[94mvertical\x1b[39m : {repèrement_vertical}\n")
                } else {
                    "".to_string()
                }
            } else {
                "".to_string()
            },
            remarques = if self.remarques.is_empty()
                && self.exploitabilité_gps == RnGPSExploitCode::Empty
            {
                "".to_string()
            } else {
                if self.remarques.is_empty() {
                    format!(
                        "\n\
                        \x1b[91m=== Remarques ===\x1b[39m\n\
                        \x1b[94mExploitabilité GPS\x1b[39m : {exploitabilité_gps:?}\n\
                        \n\
                        ",
                        exploitabilité_gps = self.exploitabilité_gps
                    )
                } else {
                    if self.exploitabilité_gps == RnGPSExploitCode::Empty {
                        format!(
                            "\n\
                            \x1b[91m=== Remarques ===\x1b[39m\n\
                            \x1b[94mRemarques\x1b[39m : {remarques}\n\
                            \n\
                            ",
                            remarques = self.remarques
                        )
                    } else {
                        format!(
                            "\n\
                            \x1b[91m=== Remarques ===\x1b[39m\n\
                            \x1b[94mRemarques\x1b[39m : {remarques}\n\
                            \x1b[94mExploitabilité GPS\x1b[39m : {exploitabilité_gps:?}\n\
                            \n\
                            ",
                            remarques = self.remarques,
                            exploitabilité_gps = self.exploitabilité_gps
                        )
                    }
                }
            },
            hors_ign = if ["100001", "100063", ""].contains(&self.hors_ign.as_str()) {
                "".to_string()
            } else {
                format!("\x1b[91m{hors_ign}\x1b[39m\n", hors_ign = self.hors_ign)
            }
        )?;
        Ok(())
    }
}
