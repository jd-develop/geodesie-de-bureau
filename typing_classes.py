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


class RNGeometryJSON(TypedDict):
    type: Literal["Point"]
    coordinates: tuple[float, float]


class RNJSON(TypedDict):
    type: Literal["Feature"]
    geometry: RNGeometryJSON
    properties: RNPropertiesJSON


class GeodeticError(Exception):
    ...

