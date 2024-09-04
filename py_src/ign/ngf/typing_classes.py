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
    id: str
    domaine: Literal["nivf", "nivo"]
    nom: str  # matricule
    no: str

    type: str
    type_info: str  # complément
    remarque: str

    diffusion: bool
    maj_date: str
    commune: str
    insee: str
    entite: str  # département en général (précisé dans entite_nature)
    entite_no: str
    entite_nature: str

    localisation: str
    carte: str  # nom de la carte IGN sur laquelle le RN est présent
    carte_no: str

    voie_suivie: str
    voie_de: str
    voie_vers: str
    voie_cote: str
    voie_pk: float

    etat: str
    action: Literal["VISITE", "DETERMINATION"]
    action_date: str
    vis_date: str
    obs_date: str
    obs_org: str
    expl_gps: str

    cg1_coord1: float
    cg1_coord2: float
    cg1_coord3: float | None
    cg1_srt: str
    cg1_prec: str | None  # UNSURE
    cg1_date: str | None  # UNSURE

    cp1_coord1: int
    cp1_coord2: int
    cp1_srt: str
    cp1_prec: str | None  # UNSURE
    cp1_coord3 : float  # ALTITUDE
    cp1_srv: str
    cp1_precv: str | None  # UNSURE
    cp1_altitude_type: str
    cp1_date: str

    cg2_coord1: float | None
    cg2_coord2: float | None
    cg2_coord3: float | None
    cg2_srt: str | None
    cg2_prec: str | None  # UNSURE
    cg2_date: str | None  # UNSURE

    cp2_coord1: int | None
    cp2_coord2: int | None
    cp2_srt: str | None
    cp2_prec: str | None  # UNSURE
    cp2_coord3 : float | None
    cp2_srv: str | None
    cp2_precv: str | None  # UNSURE
    cp2_altitude_type: str | None
    cp2_date: str | None

    g: str | None  # UNSURE
    gravi_no: str | None  # UNSURE
    gravi_srg: str | None  # UNSURE
    gravi_traitprecision: str | None  # UNSURE
    gravi_calcul_date: str | None  # UNSURE
    gravi_abs_date: str | None  # UNSURE
    gravi_rel_date: str | None  # UNSURE
    gravi_gradient: str | None  # UNSURE
    gravi_gradient_ecart_type: str | None  # UNSURE
    gravi_grad_date: str | None  # UNSURE

    freres_info: str | None  # Autres repères du triplet
    groupe_info: str | None  # UNSURE

    voisin: str | None
    voisin_distance: float | None
    voisin_domaine: Literal["nivf", "nivo"] | None

    jumeau: str | None  # Matricule rsg du site (s’il s’agit aussi d’un point géodésique)
    jumeau_no: str | None  # Numéro du point dans le site (ex: b)
    jumeau_info: str | None  # Deux infos précédentes en toutes lettres (ex: Point b du site géodésique 3155504)
    jumeau_dom: Literal["rsgf", "rsgo"] | None  # UNSURE (note: lorsque l’API sera adaptée pour les points du RGF,
                                                # il faudra ici mettre nivf et nivo)

    autre_canevas_info: str | None  # UNSURE

    support: str
    support_part: str
    rep_hori: str | None
    rep_vert: str | None

    proprio: str  # ex: "INSTITUT NATIONAL DE L'INFORMATION GEOGRAPHIQUE ET FORESTIERE (IGN)"
    proprio_artic: str | None
    proprio_sigle: str | None
    proprio_adr: str | None

    url_pdf: str
    img1: str | None
    img1_date: str | None
    img1_azim: int | None
    img1_url: int | None

    img2: str | None
    img2_date: str | None
    img2_azim: int | None
    img2_url: int | None

    groupe_img1: str | None
    groupe_img1_date: str | None
    groupe_img1_azim: int | None
    groupe_img1_url: int | None

    groupe_img2: str | None
    groupe_img2_date: str | None
    groupe_img2_azim: int | None
    groupe_img2_url: int | None

    groupe_croquis1: str | None
    groupe_croquis1_date: str | None
    groupe_croquis1_azim: int | None
    groupe_croquis1_url: int | None

    groupe_croquis2: str | None
    groupe_croquis2_date: str | None
    groupe_croquis2_azim: int | None
    groupe_croquis2_url: int | None

    groupe_type: str | None
    groupe_typecode: str | None
    groupe_typeinfo: str | None
    groupe_lieudit: str | None


ETAT_LITERAL = Literal[
    "Détruit", "Bon état", "Imprenable", "Mauvais état", "Non retrouvé", "Présumé déplacé",
    "Supposé détruit (déposé par un service local)", "Détruit après observation"
]

TYPE_LITERAL = Literal[
    "Inconnu",
    "Repère console",
    "Rivet",
    "Repère Bourdalouë",
    "Repère PLM (Chemin de fer Paris-Lyon-Méditerranée)",
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
    "Trait de crue",
    "Borne",
    "Repère SHOM (Service Hydrographique et Océanographique de la Marine)",
    "Repère fondamental",
    "Tube",
    "Repère IPG (Institut de Physique du Globe)",
    "Repère conique",
    "Repère en fonte triangulaire"
]


class BetterDict(TypedDict):  # TODO: réécrire cette classe
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
    voie_de: str | None
    voie_vers: str | None
    voie_cote: str | None
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
        "Exploitable directement par GNSS",
        "Exploitable par GNSS depuis une station excentrée",
        "Inexploitable par GNSS",
        "Exploitation par GNSS inconnue"
    ]
    geod_info: str


class RNGeometryJSON(TypedDict):
    type: Literal["Point"]
    coordinates: tuple[float, float]
    geometry_name: str


class RNJSON(TypedDict):
    type: Literal["Feature"]
    id: str
    geometry: RNGeometryJSON
    properties: RNPropertiesJSON
    bbox: dict[Literal[0, 1, 2, 3], float]


class GeodeticError(Exception):
    ...

