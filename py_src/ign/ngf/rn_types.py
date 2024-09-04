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
from typing import Literal
from typing_classes import ETAT_LITERAL, TYPE_LITERAL

RN_TYPE_CODE: dict[str, TYPE_LITERAL] = {
    "INCONNU": "Inconnu",
    "C   REPERE CONSOLE": "Repère console",
    "R   RIVET": "Rivet",
    "B   REPERE BOURDALOUE": "Repère Bourdalouë",
    "REPERE PLM (CHEMIN DE FER PARIS-LYON-MEDITERRANEE)":
        "Repère PLM (Chemin de fer Paris-Lyon-Méditerranée)",
    "REPERE MRU (MINISTERE RECONSTRUCTION URBANISME)":
        "Repère MRU (Ministère Reconstruction Urbanisme)",
    "REPERE PONTS ET CHAUSSEES": "Repère ponts et chaussées",
    "REPERE NAVIGATION": "Repère navigation",
    "REPERE VILLE DE PARIS": "Repère ville de Paris",
    "M   REPERE CYLINDRIQUE DU NIVELLEMENT GENERAL":
        "Repère cylindrique du Nivellement Général",
    "REPERE LOCAL": "Repère local",
    "REPERE HEXAGONAL": "Repère hexagonal",
    "REPERE LOCAL             REPERE DANS UN SYSTEME LOCAL":
        "Repère local, repère dans un système local",
    "ECHELLE HYDROMETRIQUE": "Échelle hydrométrique",
    "REPERE BOULE": "Repère boule",
    "REPERE ITALIEN": "Repère italien",
    "REPERE DE CRUE": "Repère de crue",
    "REPERE OCTOGONAL": "Repère octogonal",
    "REPERE CONSTRUCTION": "Repère construction",
    "REPERE EDF": "Repère EDF",
    "REPERE SNCF": "Repère SNCF",
    "REPERE CADASTRE": "Repère cadastre",
    "REPERE ALLEMAND": "Repère allemand",
    "REPERE BELGE": "Repère belge",
    "REPERE LUXEMBOURGEOIS": "Repère luxembourgeois",
    "REPERE SUISSE": "Repère suisse",
    "REPERE ESPAGNOL": "Repère espagnol",
    "REPERE VILLE DE MARSEILLE": "Repère ville de Marseille",
    "TRAIT DE CRUE": "Trait de crue",
    "BORNE": "Borne",
    "REPERE SHOM (SERVICE HYDROGRAPHIQUE ET OCEANOGRAPHIQUE DE LA MARINE)":
        "Repère SHOM (Service Hydrographique et Océanographique de la Marine)",
    "REPERE FONDAMENTAL": "Repère fondamental",
    "TUBE": "Tube",
    "REPERE IPG (INSTITUT DE PHYSIQUE DU GLOBE)":
        "Repère IPG (Institut de Physique du Globe)",
    "REPERE CONIQUE": "Repère conique",
    "REPERE EN FONTE TRIANGULAIRE": "Repère en fonte triangulaire"
}

RN_ETAT: dict[str, ETAT_LITERAL] = {  # TODO: réécrire ce dict
    "D": "Détruit",
    "E": "Bon état",
    "I": "Imprenable",
    "M": "Mauvais état",
    "N": "Non retrouvé",
    "P": "Présumé déplacé",
    "S": "Supposé détruit (déposé par un service local)",
    "Y": "Détruit après observation"
}

H_TYPE_CODE: dict[str, Literal["Altitude normale", "Altitude orthométrique", "Altitude provisoire"]] = {
    "ALTITUDE NORMALE": "Altitude normale",
    "ALTITUDE ORTHOMETRIQUE": "Altitude orthométrique",
    "ALTITUDE PROVISOIRE": "Altitude provisoire"
}


def get_rn_type(type_: str) -> TYPE_LITERAL:
    return RN_TYPE_CODE.get(type_, "Inconnu")


def get_etat_colour(full_type: str):
    if full_type == "Bon état":
        return f"\033[32m{full_type}\033[0m"
    return f"\033[31m{full_type}\033[0m"


def get_gps_exploit(gps_exploit_code: str):
    match gps_exploit_code.split():
        case ["EXPLOITABLE", "DIRECTEMENT", "PAR", _]:
            return "Exploitable directement par GNSS"
        case ["EXPLOITABLE", "PAR", _, "DEPUIS", "UNE", "STATION", "EXCENTREE"]:
            return "Exploitable par GNSS depuis une station excentrée"
        case ["INEXPLOITABLE", "PAR", _]:
            return "Inexploitable par GNSS"
        case _:
            return "Exploitation par GNSS inconnue"

