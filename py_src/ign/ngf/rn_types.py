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

RN_TYPE_CODE = {
    "000": "Inconnu",
    "001": "Repère console",
    "007": "Rivet",
    "008": "Repère Bourdaloë",
    "009": "Repère PLM (Chemins de fer Paris-Lyon-Méditerranée)",
    "010": "Repère MRU (Ministère Reconstruction Urbanisme)",
    "011": "Repère ponts et chaussées",
    "012": "Repère navigation",
    "013": "Repère ville de Paris",
    "014": "Repère cylindrique du Nivellement Général",
    "015": "Repère local",
    "016": "Repère hexagonal",
    "017": "Repère local, repère dans un système local",
    "018": "Échelle hydrométrique",
    "019": "Repère boule",
    "020": "Repère italien",
    "021": "Repère de crue",
    "022": "Repère octogonal",
    "023": "Repère construction",
    "024": "Repère EDF",
    "025": "Repère SNCF",
    "026": "Repère cadastre",
    "027": "Repère allemand",
    "028": "Repère belge",
    "029": "Repère luxembourgeois",
    "030": "Repère suisse",
    "031": "Repère espagnol",
    "032": "Repère ville de Marseille",
    "033": "Traie de crue",
    "034": "Borne",
    "035": "Repère SHOM (Service Hydrographique et Océanographique de la Marine)",
    "036": "Repère fondamental",
    "037": "Tube",
    "038": "Repère IPG (Institut de Physique du Globe)",
    "039": "Repère conique",
    "040": "Repère en fonte triangulaire"
}

RN_ETAT = {
    "D": "Détruit",
    "E": "Bon état",
    "I": "Imprenable",
    "M": "Mauvais état",
    "N": "Non retrouvé",
    "P": "Présumé déplacé",
    "S": "Supposé détruit (déposé par un service local)",
    "Y": "Détruit après observation"
}


def get_rn_type(type_: str):
    return RN_TYPE_CODE.get(type_, "000")


def get_etat_colour(full_type: str):
    if full_type == "Bon état":
        return f"\033[32m{full_type}\033[0m"
    return f"\033[31m{full_type}\033[0m"


def get_cote(cote_code: str):
    if cote_code == "G":
        return "Gauche"
    elif cote_code == "D":
        return "Droit"
    return "Milieu"

