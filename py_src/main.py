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

# GDB imports
import ign.ngf.getNgfRnInfo as ngf
# global python imports
import sys
import json
import os


def help_command(cmd: str):
    if cmd in ["quitter", "ign-nivf", "aide"]:
        with open(f"cli_help/{cmd}", "r+", encoding="UTF-8") as help_file:
            print(help_file.read())
    else:
        print(f"Erreur : commande {cmd} non reconnue.")


def get_from_insee(function_to_apply_to_numero_insee, insee):  # type: ignore
    try:
        rns = function_to_apply_to_numero_insee(insee)  # type: ignore
    except ngf.GeodeticError:
        print(
            "Erreur : numéro INSEE inconnu ou aucun RN dans la commune"
        )
        return None
    except KeyboardInterrupt:
        print("Interruption par l’utilisateur")
        return None
    except Exception as e:
        print(f"Erreur : {e.__class__.__name__} : {e}")
        return None

    if len(rns) == 5000:  # type: ignore
        print("Attention : 5000 repères récupérés. C’est la limite de "
              "l’API de l’IGN. Soit il y a effectivement 5000 repères,"
              " soit il y en a plus et certains n’apparaissent pas dans"
              " la requête.")
    return rns  # type: ignore


def sauve_matricule(rn_json):  # type: ignore
    insee = rn_json["properties"]["insee"]  # type: ignore
    nom_commune = rn_json["properties"]["commune"]  # type: ignore
    nom_fichier = os.path.expanduser(f"~/Documents/gdb/rns{insee}-{nom_commune}.geojson")
    print(f"Sauvegarde dans ~/Documents/gdb/rns{insee}-{nom_commune}.geojson")
    if not os.path.exists(nom_fichier):
        with open(nom_fichier, "w", encoding="UTF-8") as geojson1:
            geojson1.write(
                "{\n"
                "    \"type\": \"featureCollection\",\n"
                "    \"features\": [],\n"
                "    \"crs\": {\n"
                "        \"type\": \"name\",\n"
                "        \"properties\": {\n"
                "            \"name\": \"urn:ogc:def:crs:EPSG::4326\"\n"
                "        }\n"
                "    }\n"
                "}\n"
            )

    with open(nom_fichier, "r+", encoding="UTF-8") as geojson:
        print(geojson.read())
        data = json.loads(geojson.read())  # FIXME
        found = False
        for feature in data["features"]:
            if feature["properties"]["matricule"] == rn_json["properties"]["matricule"]:  # type: ignore
                feature["properties"] = rn_json["properties"]  # type: ignore
                feature["id"] = rn_json["id"]  # type: ignore
                feature["geometry"] = rn_json["geometry"]  # type: ignore
                feature["bbox"] = rn_json["bbox"]  # type: ignore
                found = True
                break
        if not found:
            data["features"].append(rn_json)
        json.dump(data, geojson, indent=4)


print("Géodésie de Bureau")
print()
print("Ce programme est un logiciel libre, vous pouvez le redistribuer et/ou")
print("le modifier sous les termes de la Licence GNU General Public License")
print("publiée par la Free Software Foundation, dans sa version 3 ou (à votre")
print("guise) n’importe quelle version ultérieure")
print()
print("Tapez 'aide' pour obtenir de l’aide, et 'quitter' pour quitter le "
      "programme")
print()


while True:
    try:
        command = input("gdb> ").strip().split(maxsplit=1)
    except KeyboardInterrupt:
        print("Interruption par l’utilisateur")
        sys.exit()
    except EOFError:
        print("Interruption par l’utilisateur")
        sys.exit()
    if command == []:
        continue
    # print(command)
    if command[0] in ["exit", "quitter"]:
        sys.exit()
    elif command[0] in ["aide", "help"]:
        if len(command) == 1:
            with open("cli_help/help", "r+", encoding="UTF-8") as cli_help:
                print(cli_help.read())
        else:
            help_command(command[1])
    elif command[0] in ["ign-nivf", "ign-nvif"]:
        if len(command) == 1:
            print("Erreur : sous-commande attendue")
            continue
        subcommand = command[1].split(maxsplit=1)
        if subcommand[0] == "recupinsee":
            if len(subcommand) == 1:
                print("Erreur : numéro INSEE d’une commune attendu")
                continue
            insee = subcommand[1]

            rns = get_from_insee(lambda x: list(map(ngf.better_dict, ngf.dicts_from_insee(x))), insee)  # type: ignore
            if rns is None:
                continue

            for rn in rns:
                ngf.print_fiche(rn)
                print()
        elif subcommand[0] == "sauveinsee":
            if len(subcommand) == 1:
                print("Erreur : numéro INSEE d’une commune ou « plusieurs » attendu")
                continue
            insee = subcommand[1]

            if insee == "plusieurs":
                print("Quittez avec « stop ».")
                while (insee := input("Entrez un numéro INSEE : ")) != "stop":
                    rns = get_from_insee(ngf.better_dicts_from_insee, insee)

                    if rns is None:
                        continue

                    nom_commune = rns["features"][0]["properties"]["commune"]
                    print(f"Sauvegarde dans ~/Documents/gdb/rns{insee}-{nom_commune}.geojson")
                    with open(
                        os.path.expanduser(f"~/Documents/gdb/rns{insee}-{nom_commune}.geojson"),
                        "w+",
                        encoding="UTF-8"
                    ) as insee_file:
                        json.dump(rns, insee_file, indent=4)
            else:
                rns = get_from_insee(ngf.better_dicts_from_insee, insee)

                if rns is None:
                    continue

                nom_commune = rns["features"][0]["properties"]["commune"]
                print(f"Sauvegarde dans ~/Documents/gdb/rns{insee}-{nom_commune}.geojson")
                with open(
                    os.path.expanduser(f"~/Documents/gdb/rns{insee}-{nom_commune}.geojson"),
                    "w+",
                    encoding="UTF-8"
                ) as insee_file:
                    json.dump(rns, insee_file, indent=4)

        elif subcommand[0] in ["recupmatricule", "sauvematricule"]:
            if len(subcommand) == 1:
                print("Erreur : matricule de repère de nivellement ou « plusieurs » attendu")
                continue

            matricule = subcommand[1]
            if matricule == "plusieurs":
                print("Quittez avec « stop »")
                while (matricule := input("Matricule : ")) != "stop":
                    try:
                        if subcommand[0] == "recupmatricule":
                            rn_dict = ngf.better_dict(ngf.dict_from_matricule(matricule))
                        else:
                            rn_dict = ngf.better_dict_geojson(ngf.dict_from_matricule(matricule))
                        lien_direct = ngf.get_lien_direct_from_matricule(ngf.formatted_matricule(matricule))
                    except ngf.GeodeticError:
                        print("Erreur : matricule inconnu ou non-unique")
                        continue
                    except KeyboardInterrupt:
                        print("Interruption par l’utilisateur")
                        continue
                    except Exception as e:
                        print(f"Erreur : {e.__class__.__name__} : {e}")
                        continue
                    if subcommand[0] == "recupmatricule":
                        ngf.print_fiche(rn_dict, lien_direct)  # type: ignore
                        print()
                    else:
                        sauve_matricule(rn_dict)
            else:
                try:
                    if subcommand[0] == "sauvematricule":
                        rn_dict = ngf.better_dict_geojson(ngf.dict_from_matricule(matricule))
                    else:
                        rn_dict = ngf.better_dict(ngf.dict_from_matricule(matricule))
                    lien_direct = ngf.get_lien_direct_from_matricule(ngf.formatted_matricule(matricule))
                except ngf.GeodeticError:
                    print("Erreur : matricule inconnu ou non-unique")
                    continue
                except KeyboardInterrupt:
                    print("Interruption par l’utilisateur")
                    continue
                except Exception as e:
                    print(f"Erreur : {e.__class__.__name__} : {e}")
                    continue
                if subcommand[0] == "recupmatricule":
                    ngf.print_fiche(rn_dict, lien_direct)  # type: ignore
                    print()
                else:
                    sauve_matricule(rn_dict)
        else:
            print(f"Erreur : commande `ign-nivf {subcommand[0]}' inconnue.")
            continue
    else:
        print(f"Erreur : commande `{command[0]}' inconnue.")

