#!/usr/bin/env python3
# -*- coding:utf-8 -*-

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
"""getNgfRnInfo.py - Gets the informations about a Repère de nivellement
                     (NGF, i.e. metropolitan France including Corsica)
"""
import requests
import json
from typing_classes import *
from rn_types import *
from requests_responses_parser import SearchParser
import math

RED = "\033[91m"
GREEN = "\033[92m"
BLUE = "\033[94m"
RESET = "\033[00m"

URL = "https://geodesie.ign.fr/fiches/index.php?module=e&action=visugeod"
HEADERS = {"content-type": "application/x-www-form-urlencoded", "Accept-Charset": "UTF-8"}


def limit_dms_coord_for_bbox(dms_coord: str):
    """2.1749 -> 2.1, 48.8039 -> 48.8, -5.0914 -> -5.1"""
    floor = math.floor(float(dms_coord) * 10)
    whole = str(floor)[:-1]
    decimal = str(floor)[-1]
    if not whole:
        whole = "0"
    if decimal == "0" or not decimal:
        return whole
    return whole + "." + decimal


def find_rn_dict_from_bbox(bbox_json: list[RNJSON], matricule: str) -> RNJSON:
    for rn_json in bbox_json:
        if rn_json["properties"]["rn_nom"] == matricule:
            return rn_json
    raise GeodeticError("Unable to find the Repère de nivellement in the bbox list")


def is_valid_index_choice(choice: str, list_len: int):
    try:
        return 0 <= int(choice) < list_len
    except ValueError:
        return False


def find_matricule_to_use_from_list(matricule_input: str, reperes_found: list[tuple[int, str]]) -> str:
    if len(reperes_found) == 1:
        return reperes_found[0][1]
    for _, matricule in reperes_found:
        if matricule == matricule_input:
            return matricule_input
    print("Repères found: ")
    i = 0
    for i, (cid, matricule) in enumerate(reperes_found):
        print(f"{i}: {matricule} (id {cid})")

    while not is_valid_index_choice(
            choix := input(f"Your choice (0-{i}): "),
            len(reperes_found)
    ):
        print(f"Please enter a valid choice from 0 to {i}")

    return reperes_found[int(choix)][1]


def print_fiche(rn_json: RNJSON):
    """Affiche toutes les informations sous une forme compréhensible dans la console"""
    prn_json = better_dict(rn_json)
    print(RED + "=============== Repère de nivellement ===============" + RESET)
    print()
    print(BLUE + f"Fiche en ligne{RESET} : {prn_json['fiche_url']}")
    print()
    print(BLUE + f"Matricule{RESET} : {prn_json['matricule']}")
    print(BLUE + f"Système altimétrique{RESET} : {prn_json['systeme_altimetrique']}")
    print(BLUE + f"Altitude{RESET} : {prn_json['altitude']}m ({prn_json['altitude_type']})")
    if prn_json["altitude_complementaire"]:
        print(BLUE + f"Altitude{RESET} : {prn_json['altitude_complementaire']}m (Altitude complémentaire)")

    print()
    print(RED + "=== Dernière visite et observation ===" + RESET)
    print(BLUE + f"Année de dernière observation{RESET} : {prn_json['derniere_observation']}")
    print(BLUE + f"Année de nouveau calcul{RESET} : {prn_json['nouveau_calcul']}")
    print(BLUE + f"Dernière visite{RESET} : {prn_json['derniere_visite']}")
    assert isinstance(prn_json["etat"], str)
    print(BLUE + f"État{RESET} : {get_etat_colour(prn_json['etat'])}")

    print()
    print(RED + "=== Type ===" + RESET)
    print(BLUE + f"Type{RESET} : {prn_json['type']}")
    if prn_json["type_complement_avec_canex"]:
        print(BLUE + f"Complément{RESET} : {prn_json['type_complement_avec_canex']}")

    print()
    print(RED + "=== Coordonnées DMS ===" + RESET)
    # print("Système : RGF93 v1 (ETRS89) - Ellipsoïde : IAG GRS 1980")
    print(BLUE + f"Longitude (dms){RESET} : {prn_json['longitude']}")
    print(BLUE + f"Latitude (dms){RESET} : {prn_json['latitude']}")
    print(RED + "=== Coordonnées en mètres ===" + RESET)
    # print("Système : RGF93 v1 (ETRS89) - Projection : LAMBERT-93")
    print(BLUE + f"E (m){RESET} : {prn_json['e']}")
    print(BLUE + f"N (m){RESET} : {prn_json['n']}")

    print()
    print(RED + "=== Localisation ===" + RESET)
    print(BLUE + f"Département{RESET} : {prn_json['departement']}")
    print(BLUE + f"Numéro INSEE{RESET} : {prn_json['insee']}")
    print(BLUE + f"Commune{RESET} : {prn_json['commune']}")
    print(BLUE + f"Voie suivie{RESET} : {prn_json['voie_suivie']}")
    print(f"|- {BLUE}de{RESET}       : {prn_json['voie_de']}")
    print(f"|- {BLUE}à{RESET}        : {prn_json['voie_vers']}")
    print(f"|- {BLUE}côté{RESET}     : {prn_json['voie_cote']}")
    if prn_json["voie_pk"]:
        print(f"|- {BLUE}PK{RESET}       : {prn_json['voie_pk']} km")
    if prn_json["distance"]:
        print(BLUE + f"Distance{RESET}     : {prn_json['distance']} km")
        print(f"|- {BLUE}du repère{RESET} : {prn_json['du_repere']}")
    print(BLUE + f"Localisation{RESET} : {prn_json['localisation']}")

    print()
    print(RED + "=== Support === " + RESET)
    print(BLUE + f"Support{RESET} : {prn_json['support']}")
    print(BLUE + f"Partie du support{RESET} : {prn_json['partie_support']}")
    print(BLUE + f"Repèrements{RESET} :")
    print(f"|- {BLUE}horizontal{RESET} : {prn_json['reperement_horizontal']}")
    print(f"|- {BLUE}vertical{RESET}   : {prn_json['reperement_vertical']}")

    if prn_json["hors_ign"] not in ["100001", "100063", "", None]:
        print()
        print(RED + str(prn_json["hors_ign"]) + RESET)
    # TODO triplets de nivellement


def choose_rn_from_matricule(matricule_input: str) -> str:
    """Using the matricule provided by the user, searches for corresponding
       matricules, lets the user choose one or
       chooses one automatically if there is no ambiguity"""
    assert len(matricule_input) != 0
    assert "|" not in matricule_input
    matricule_input = matricule_input.replace("’", "'").strip()

    data = {"repere_ajax": matricule_input, "identifiant_visugeod": "identificateur_repere"}

    response = requests.post(URL, headers=HEADERS, data=data)
    parser = SearchParser()
    parser.feed(response.text)
    reperes_found = parser.rn_list

    if len(reperes_found) == 0:
        raise GeodeticError(
            f"Could not find Repère de Nivellement from matricule {matricule_input}"
        )

    return find_matricule_to_use_from_list(matricule_input, reperes_found)


def dict_from_matricule(matricule_to_use: str) -> RNJSON:
    """Returns the dict of the Repère de Nivellement from its matricule"""
    matricule_with_double_primes = matricule_to_use.replace("'", "''")
    data = {"h_recherche": f"repere|{matricule_with_double_primes}", "t": "france"}
    response = requests.post(URL, headers=HEADERS, data=data)
    rn_basic_infos = response.text.split("\n")[0]
    if rn_basic_infos == "1":
        raise GeodeticError(
            f"Could not find Repère de Nivellement from matricule {matricule_to_use}"
        )

    dms_coords_raw = rn_basic_infos.split("|")[0].split()
    long, lat = map(limit_dms_coord_for_bbox, dms_coords_raw)

    bbox_url = f"https://geodesie.ign.fr/ripgeo/fr/api/nivrn/bbox/{long}/{lat}/json/"
    bbox_json_raw = requests.post(bbox_url).text

    bbox_json = json.loads(bbox_json_raw)
    rn_json = find_rn_dict_from_bbox(bbox_json["features"], matricule_to_use)
    return rn_json


def better_dict(rn_json: RNJSON):
    """Returns the useful information contained in the dictionnary"""
    fiche_url = f"https://geodesie.ign.fr/fiches/index.php?module=e&action=fichepdf&source=gp&rn_cid=" \
                f"{rn_json['properties']['rn_cid']}&geo_cid=0"
    if rn_json["properties"]["nivf_rea_code"] == 2:
        annee_syst_altimetrique = "1969"
    else:
        annee_syst_altimetrique = "1978"
    type_complement = rn_json["properties"]["rn_type_compl"]
    canex_info = rn_json["properties"]["canex_info"]
    if type_complement and canex_info:
        type_complement_avec_canex = type_complement + ", " + canex_info
    elif type_complement:
        type_complement_avec_canex = type_complement
    elif canex_info:
        type_complement_avec_canex = canex_info
    else:
        type_complement_avec_canex = ""
    return {
        "matricule": rn_json["properties"]["rn_nom"],
        "cid": rn_json["properties"]["rn_cid"],
        "fiche_url": fiche_url,
        "systeme_altimetrique": f"NGF-IGN {annee_syst_altimetrique}",
        "altitude": rn_json["properties"]["altitude"],
        "altitude_complementaire": rn_json["properties"]["altitude_complementaire"],
        "altitude_type": H_TYPE_CODE[rn_json["properties"]["h_type_code"]],

        "derniere_observation": rn_json["properties"]["rn_obs_date"],
        "nouveau_calcul": rn_json["properties"]["trg_annee"],
        "derniere_visite": rn_json["properties"]["rn_vis_date"],
        "etat": RN_ETAT[rn_json["properties"]["rn_etat_code"]],

        "type": get_rn_type(rn_json["properties"]["rn_type_code"]),
        "type_complement": rn_json["properties"]["rn_type_compl"],
        "canex_info": rn_json["properties"]["canex_info"],
        "type_complement_avec_canex": type_complement_avec_canex,

        "longitude": rn_json["geometry"]["coordinates"][0],
        "latitude": rn_json["geometry"]["coordinates"][1],
        "e": rn_json["properties"]["e"],
        "n": rn_json["properties"]["n"],

        "departement": rn_json["properties"]["departement_code"],
        "insee": rn_json["properties"]["insee"],
        "commune": rn_json["properties"]["commune_nom"],
        "voie_suivie": rn_json["properties"]["voie_suivie"],
        "voie_de": rn_json["properties"]["voie_de"],
        "voie_vers": rn_json["properties"]["voie_vers"],
        "voie_cote": get_cote(rn_json["properties"]["voie_cote"]),
        "voie_pk": rn_json["properties"]["voie_pk"],
        "distance": rn_json["properties"]["distance"],
        "du_repere": rn_json["properties"]["rn_proche_nom"],
        "localisation": rn_json["properties"]["localisation"],

        "support": rn_json["properties"]["support"],
        "partie_support": rn_json["properties"]["support_partie"],
        "reperement_horizontal": rn_json["properties"]["reper_horiz"],
        "reperement_vertical": rn_json["properties"]["reper_vertical"],

        "hors_ign": rn_json["properties"]["hors_ign"]
    }


if __name__ == "__main__":
    matricule = input("Matricule of the Repère de nivellement: ")
    print_fiche(dict_from_matricule(choose_rn_from_matricule(matricule)))
    # I usually test those benchmarks, for their particularities:
    # M.AC - 0-VIII (repère fondamental)
    # T'.D.S3 - 17
    # N.P.K3Q3 - 56
    # N.P.K3Q3 - 57
    # N.P.K3Q3 - 58
    # N.P.K3Q3 - 58-I
    # T’.D.S3 - 102a
    # M’.A.K3 - 1
    # M".A.K3L3 - 15-I
    # FM" - 3-VIII
    # FM" - 3-V

