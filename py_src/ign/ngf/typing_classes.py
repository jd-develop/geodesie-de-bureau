#!/usr/bin/env python3
# -*- coding=utf-8 -*-

# Géodésie de Bureau - keep a list of the geodetic benchmarks you’ve seen!
# Copyright (C) 2024  Jean Dubois

# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.

# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.

# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.
from typing import TypedDict, Literal


class RNPropertiesJSON(TypedDict):
    image_name: str
    rn_type_code: str
    nivf_ref_en_code: int
    nivf_rea_code: int
    nivf_ref_lp_code: int
    h_type_code: int
    rn_etat_code: str
    rn_action_code: str
    rn_voie_cote_code: str
    rn_gps_eploit_code: str
    hors_ign: str
    departement_code: str
    rn_cid: int
    rn_nom: str
    rn_type_compl: str
    insee: str
    commune_nom: str
    localisation: str
    carte_no: str
    voie_suivie: str
    voie_de: str
    voie_vers: str
    voie_cote: str
    voie_pk: str | None
    distance: str | None
    rn_proche_nom: str | None
    e: str
    n: str
    lambda_dms: str
    phi_dms: str
    support: str
    support_partie: str
    reper_horiz: str
    reper_vertical: str
    altitude: str
    altitude_complementaire: str
    trg_annee: str
    rn_obs_date: str
    rn_vis_date: str
    remarque: str
    triplet_cid: str 
    geod_info: str
    canex_info: str
    rn_primordial_cid: ...
    sit_no: ...
    ptg_croquis_lettre: str
    sit_info: str


ETAT_LITERAL = Literal[
    "Détruit", "Bon état", "Imprenable", "Mauvais état", "Non retrouvé", "Présumé déplacé",
    "Supposé détruit (déposé par un service local)", "Détruit après observation"
]

TYPE_LITERAL = Literal[
    "Inconnu",
    "Repère console",
    "Rivet",
    "Repère Bourdalouë",
    "Repère PLM (Chemins de fer Paris-Lyon-Méditerranée)",
    "Repère MRU (Ministère Reconstruction Urbanisme)",
    "Repère ponts et chaussées",
    "Repère navigation",
    "Repère ville de Paris",
    "Repère cylindrique du Nivellement Général",
    "Repère local",
    "Repère hexagonal",
    "Repère local, repère dans un système local",
    "Échelle hydrométrique",
    "Repère boule",
    "Repère italien",
    "Repère de crue",
    "Repère octogonal",
    "Repère construction",
    "Repère EDF",
    "Repère SNCF",
    "Repère cadastre",
    "Repère allemand",
    "Repère belge",
    "Repère luxembourgeois",
    "Repère suisse",
    "Repère espagnol",
    "Repère ville de Marseille",
    "Traie de crue",
    "Borne",
    "Repère SHOM (Service Hydrographique et Océanographique de la Marine)",
    "Repère fondamental",
    "Tube",
    "Repère IPG (Institut de Physique du Globe)",
    "Repère conique",
    "Repère en fonte triangulaire"
]


class BetterDict(TypedDict):
    matricule: str
    cid: int
    fiche_url: str
    systeme_altimetrique: Literal["NGF-IGN 1969", "NGF-IGN 1978"]
    altitude: str
    altitude_complementaire: str
    altitude_type: Literal["Altitude normale", "Altitude orthométrique", "Altitude provisoire"]
    derniere_observation: str
    nouveau_calcul: str
    derniere_visite: str
    etat: ETAT_LITERAL
    type: TYPE_LITERAL
    type_complement: str
    canex_info: str
    type_complement_avec_canex: str

    longitude: float
    latitude: float
    e: str
    n: str

    departement: str
    insee: str
    commune: str
    voie_suivie: str
    voie_de: str
    voie_vers: str
    voie_cote: str
    voie_pk: str | None
    distance: str | None
    du_repere: str | None
    localisation: str
    support: str
    partie_support: str
    reperement_horizontal: str
    reperement_vertical: str

    hors_ign: str
    remarques: str
    exploitabilite_gps: Literal[
        "Exploitable directement par GPS",
        "Exploitable par GPS depuis une station excentrée",
        "Inexploitable par GPS",
        "Exploitation par GPS inconnue"
    ]
    geod_info: str


class RNGeometryJSON(TypedDict):
    type: Literal["Point"]
    coordinates: tuple[float, float]


class RNJSON(TypedDict):
    type: Literal["Feature"]
    geometry: RNGeometryJSON
    properties: RNPropertiesJSON


class GeodeticError(Exception):
    ...

