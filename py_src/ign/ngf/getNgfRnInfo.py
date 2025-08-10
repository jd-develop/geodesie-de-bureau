#!/usr/bin/env python3
# -*- coding:utf-8 -*-

# Géodésie de Bureau - keep a list of the geodetic benchmarks you’ve seen!
# Copyright (C) 2024-2025  Jean Dubois

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
                     (NGF-nivf, i.e. metropolitan France including Corsica)
"""
import requests
from ign.ngf.rn_types import *
from ign.ngf.typing_classes import *

RED = "\033[91m"
GREEN = "\033[92m"
BLUE = "\033[94m"
RESET = "\033[00m"

URL = "https://data.geopf.fr/wfs"

def get_params_from_matricule(matricule: str) -> dict[str, str]:
    return {
        "SERVICE": "WFS",
        "VERSION": "2.0.0",
        "REQUEST": "GetFeature",
        "TYPENAME": "GEODESIE:data_geod",
        "OUTPUTFORMAT": "application/json",
        "cql_filter": f"nom='{matricule}' and domaine='nivf'"
    }


def get_lien_direct_from_matricule(matricule: str) -> str:
    return \
        f"\x1b]8;;https://data.geopf.fr/wfs?SERVICE=WFS&VERSION=2.0.0&" \
        f"REQUEST=GetFeature&TYPENAME=GEODESIE:data_geod&OUTPUTFORMAT=" \
        f"application/json&cql_filter=nom=%27{matricule}%27+and+domaine" \
        f"=%27nivf%27\x1b\\Ici\x1b]8;;\x1b\\"


def get_params_from_commune(insee: str) -> dict[str, str]:
    return {
        "SERVICE": "WFS",
        "VERSION": "2.0.0",
        "REQUEST": "GetFeature",
        "TYPENAME": "GEODESIE:data_geod",
        "OUTPUTFORMAT": "application/json",
        "cql_filter": f"insee='{insee}' and domaine='nivf'"
    }


def print_fiche(rn_json: BetterDict, lien_direct: str | None = None):
    """Affiche toutes les informations sous une forme compréhensible dans la console"""
    print(RED + "=============== Repère de nivellement ===============" + RESET)
    print()
    print(BLUE + f"Fiche en ligne{RESET} : \x1b]8;;{rn_json['fiche_url']}\x1b\\{rn_json['fiche_url']}\x1b]8;;\x1b\\")
    if lien_direct is not None:
        print(BLUE + f"Geojson brut en ligne{RESET} : {lien_direct}")
    print()
    print(BLUE + f"Matricule{RESET} : {rn_json['matricule']}")
    print(BLUE + f"Système altimétrique{RESET} : {rn_json['systeme_altimetrique']}")
    print(BLUE + f"Altitude{RESET} : {rn_json['altitude']}m ({rn_json['altitude_type']})")

    print()
    print(RED + "=== Dernière visite et observation ===" + RESET)
    print(BLUE + f"Année de dernière observation{RESET} : {rn_json['derniere_observation']}")
    print(BLUE + f"Année de nouveau calcul{RESET} : {rn_json['nouveau_calcul']}")
    print(BLUE + f"Dernière visite{RESET} : {rn_json['derniere_visite']}")
    print(BLUE + f"État{RESET} : {get_etat_colour(rn_json['etat'])}")

    print()
    print(RED + "=== Type ===" + RESET)
    print(BLUE + f"Type{RESET} : {rn_json['type']}")
    if rn_json["type_complement_avec_canex"]:
        print(BLUE + f"Complément{RESET} : {rn_json['type_complement_avec_canex']}")

    print()
    print(RED + "=== Coordonnées DMS ===" + RESET)
    print(rn_json["dms_syst_et_ellipsoide"])
    print(BLUE + f"Longitude (dms){RESET} : {rn_json['longitude_dms']}")
    print(BLUE + f"Latitude (dms){RESET} : {rn_json['latitude_dms']}")
    print(BLUE + f"Longitude {RESET} : {rn_json['longitude']}")
    print(BLUE + f"Latitude {RESET} : {rn_json['latitude']}")
    print(RED + "=== Coordonnées en mètres ===" + RESET)
    print(rn_json["en_syst_et_ellipsoide"])
    print(BLUE + f"E (m){RESET} : {rn_json['e']}")
    print(BLUE + f"N (m){RESET} : {rn_json['n']}")

    print()
    print(RED + "=== Localisation ===" + RESET)
    print(BLUE + f"{rn_json['entite_nature']}{RESET} : {rn_json['entite']} ({rn_json['entite_no']})")
    print(BLUE + f"Numéro INSEE{RESET} : {rn_json['insee']}")
    print(BLUE + f"Commune{RESET} : {rn_json['commune']}")
    print(BLUE + f"Voie suivie{RESET} : {rn_json['voie_suivie']}")
    print(f"|- {BLUE}de{RESET}       : {rn_json['voie_de']}")
    print(f"|- {BLUE}à{RESET}        : {rn_json['voie_vers']}")
    print(f"|- {BLUE}côté{RESET}     : {rn_json['voie_cote']}")
    if rn_json["voie_pk"] is not None:
        print(f"|- {BLUE}PK{RESET}       : {rn_json['voie_pk']} km")
    if rn_json["distance"] is not None:
        print(BLUE + f"Distance{RESET}     : {rn_json['distance']} km")
        print(f"|- {BLUE}du repère{RESET} : {rn_json['du_repere']}")
    print(BLUE + f"Localisation{RESET} : {rn_json['localisation']}")

    print()
    print(RED + "=== Support === " + RESET)
    print(BLUE + f"Support{RESET} : {rn_json['support']}", end="")
    if rn_json["geod_info"] != "":
        print(f" ({rn_json['geod_info']})")
    else:
        print()

    print(BLUE + f"Partie du support{RESET} : {rn_json['partie_support']}")
    print(BLUE + f"Repèrements{RESET} :")
    print(f"|- {BLUE}horizontal{RESET} : {rn_json['reperement_horizontal']}")
    print(f"|- {BLUE}vertical{RESET}   : {rn_json['reperement_vertical']}")

    print()
    print(RED + "=== Remarques ===" + RESET)
    if rn_json["remarques"] != "":
        print(rn_json["remarques"])

    print(rn_json["exploitabilite_gnss"])

    if rn_json["hors_ign"]:
        print(RED + "Ce repère n’a pas été observé par l’IGN." + RESET)

    if rn_json["triplet"] is not None and rn_json["triplet"] != "":
        print(BLUE + f"Ce repère appartient à un triplet.{RESET} Liste des repères du triplet :", end=" ")
        print(rn_json["triplet"])


def formatted_matricule(matricule_to_use: str) -> str:
    """
    Returns a matricule usable for the API from a matricule entered by the
    user
    """
    return matricule_to_use.replace("’", "'") \
               .replace("''", '"') \
               .replace("'", "''") \
               .strip()


def dict_from_matricule(matricule_to_use: str) -> RNJSON:
    """Returns the dict of the Repère de Nivellement from its matricule"""

    matricule_with_double_primes = formatted_matricule(matricule_to_use)

    response = requests.get(URL, params=get_params_from_matricule(matricule_with_double_primes))
    response_json = response.json()
    if response_json["numberMatched"] == 0:
        raise GeodeticError(
            f"Could not find Repère de Nivellement from matricule {matricule_to_use}"
        )
    if response_json["numberMatched"] > 1:
        raise GeodeticError(
            f"Nonunique matricule!"
        )

    rn_json = response_json["features"][0]
    return rn_json


def dicts_from_insee(insee: str) -> list[RNJSON]:
    """
    Returns a list of all the Repères de Nivellement of the commune number
    `insee'
    """

    response = requests.get(URL, params=get_params_from_commune(insee))
    response_json = response.json()
    if response_json["numberMatched"] == 0:
        raise GeodeticError(
            f"Commune '{insee}' not found or no Repère de Nivellement in commune '{insee}'."
        )

    rns_json = response_json["features"]
    return rns_json


def better_dicts_from_insee(insee: str):
    """Renvoie un geojson avec les betterdicts comme features"""
    response = requests.get(URL, params=get_params_from_commune(insee))
    response_json = response.json()

    if response_json["numberMatched"] == 0:
        raise GeodeticError(
            f"Commune '{insee}' not found or no Repère de Nivellement in commune '{insee}'."
        )

    response_json["features"] = list(map(better_dict_geojson, response_json["features"]))
    return response_json


def better_dict_geojson(rn_json: RNJSON):
    """Same as better_dict but keeps the GEOJSON structure"""
    new_json = rn_json.copy()
    new_json["properties"] = better_dict(rn_json)  # type: ignore  # FIXME
    return new_json


def better_dict(rn_json: RNJSON) -> BetterDict:
    """Returns the useful information contained in the dictionnary"""
    fiche_url = rn_json["properties"]["url_pdf"]
    syst_altimetrique = get_systeme_altimetrique(rn_json["properties"]["cp1_srv"])
    type_complement = rn_json["properties"]["type_info"]
    canex_info = rn_json["properties"]["autre_canevas_info"]
    if type_complement and canex_info:
        type_complement_avec_canex = type_complement + ", " + canex_info
    elif type_complement:
        type_complement_avec_canex = type_complement
    elif canex_info:
        type_complement_avec_canex = canex_info
    else:
        type_complement_avec_canex = ""

    rep_hori = rn_json["properties"]["rep_hori"]
    rep_vert = rn_json["properties"]["rep_vert"]
    geod_info = rn_json["properties"]["jumeau_info"]
    triplet_brut = rn_json["properties"]["freres_info"]
    if triplet_brut is None:
        triplet = None
    else:
        triplet = triplet_brut.strip().replace(" , ", ", ")

    return {
        "matricule": rn_json["properties"]["nom"],
        "cid": int(rn_json["properties"]["id"]),
        "fiche_url": fiche_url,
        "systeme_altimetrique": syst_altimetrique,
        "altitude": rn_json["properties"]["cp1_coord3"],
        "altitude_type": get_altitude_type(rn_json["properties"]["cp1_altitude_type"]),

        "derniere_observation": rn_json["properties"]["obs_date"],
        "nouveau_calcul": rn_json["properties"]["cp1_date"],
        "derniere_visite": rn_json["properties"]["vis_date"],
        "etat": RN_ÉTAT[rn_json["properties"]["etat"]],

        "type": RN_TYPE_CODE[rn_json["properties"]["type"]],
        "type_complement": rn_json["properties"]["type_info"],
        "canex_info": rn_json["properties"]["autre_canevas_info"],
        "type_complement_avec_canex": type_complement_avec_canex,

        "longitude": rn_json["properties"]["cg1_coord1"],
        "latitude": rn_json["properties"]["cg1_coord2"],
        "longitude_dms": rn_json["properties"]["cg1_coord1_dms"],
        "latitude_dms": rn_json["properties"]["cg1_coord2_dms"],
        "dms_syst_et_ellipsoide": rn_json["properties"]["cg1_srt"],
        "e": rn_json["properties"]["cp1_coord1"],
        "n": rn_json["properties"]["cp1_coord2"],
        "en_syst_et_ellipsoide": rn_json["properties"]["cp1_srt"],

        "entite": rn_json["properties"]["entite"],
        "entite_no": rn_json["properties"]["entite_no"],
        "entite_nature": rn_json["properties"]["entite_nature"],
        "insee": rn_json["properties"]["insee"],
        "commune": rn_json["properties"]["commune"],
        "voie_suivie": rn_json["properties"]["voie_suivie"],
        "voie_de": rn_json["properties"]["voie_de"],
        "voie_vers": rn_json["properties"]["voie_vers"],
        "voie_cote": rn_json["properties"]["voie_cote"],
        "voie_pk": rn_json["properties"]["voie_pk"],
        "distance": rn_json["properties"]["voisin_distance"],
        "du_repere": rn_json["properties"]["voisin"],
        "localisation": rn_json["properties"]["localisation"],

        "support": rn_json["properties"]["support"],
        "partie_support": rn_json["properties"]["support_part"],
        "reperement_horizontal": rep_hori if rep_hori is not None else "",
        "reperement_vertical": rep_vert if rep_vert is not None else "",

        "hors_ign": rn_json["properties"]["obs_org"] not in ["100001", "100063"],
        "remarques": rn_json["properties"]["remarque"].strip(),
        "exploitabilite_gnss": get_gnss_exploit(rn_json["properties"]["expl_gps"]),
        "geod_info": geod_info if geod_info is not None else "",

        "triplet": triplet
    }


if __name__ == "__main__":
    if input("Matricule or INSEE? [M/i] ") in ["i", "I"]:
        insee = input("INSEE number of a commune: ")
        rns = map(better_dict, dicts_from_insee(insee))
        for rn in rns:
            print_fiche(rn)
            print()
    else:
        matricule = input("Matricule of the Repère de nivellement: ")
        print_fiche(better_dict(dict_from_matricule(matricule)))
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
    # R'.C.P3 - 155 (État : imprenable)
    # B.A.L3 - 161 (État : présumé déplacé)
