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


def help_command(cmd: str):
    if cmd in ["quitter", "ign-nivf", "aide"]:
        with open(f"cli_help/{cmd}", "r+", encoding="UTF-8") as help_file:
            print(help_file.read())
    else:
        print(f"Erreur : commande {cmd} non reconnue.")


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

            try:
                rns = map(ngf.better_dict, ngf.dicts_from_insee(insee))
            except ngf.GeodeticError:
                print(
                    "Erreur : numéro INSEE inconnu ou aucun RN dans la commune"
                )
                continue
            except KeyboardInterrupt:
                print("Interruption par l’utilisateur")
                continue
            except Exception as e:
                print(f"Erreur : {e.__class__.__name__} : {e}")
                continue

            for rn in rns:
                ngf.print_fiche(rn)
                print()
        elif subcommand[0] == "recupmatricule":
            if len(subcommand) == 1:
                print("Erreur : matricule de repère de nivellement attendu")
                continue
            matricule = subcommand[1]
            try:
                ngf.print_fiche(
                    ngf.better_dict(
                        ngf.dict_from_matricule(
                            matricule
                        )
                    ),
                    ngf.get_lien_direct_from_matricule(
                        ngf.formatted_matricule(
                            matricule
                        )
                    )
                )
                print()
            except ngf.GeodeticError:
                print("Erreur : matricule inconnu ou non-unique")
            except KeyboardInterrupt:
                print("Interruption par l’utilisateur")
            except Exception as e:
                print(f"Erreur : {e.__class__.__name__} : {e}")
        else:
            print(f"Erreur : commande `ign-nivf {command[1]}' inconnue.")
            continue
    else:
        print(f"Erreur : commande `{command[0]}' inconnue.")

