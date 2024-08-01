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

"""Write save - create and write into save file"""
# import json
import platform
import os


def _determine_config_directory():
    """And create it if it doesn’t exists"""
    system = platform.system()
    if system == "Darwin":  # macOS
        root_config_directory = os.path.abspath(os.path.expanduser("~/Library/Preferences/org.jd-develop.geodesie/"))
        if not os.path.isdir(root_config_directory):
            os.mkdir(root_config_directory)
    elif system == "Windows":  # Windows
        jd_develop_directory = os.path.expandvars('%APPDATA%\\jd-develop\\')
        root_config_directory = os.path.abspath(os.path.expandvars('%APPDATA%\\jd-develop\\geodesie\\'))
        if not os.path.isdir(jd_develop_directory):
            os.mkdir(jd_develop_directory)
        if not os.path.isdir(root_config_directory):
            os.mkdir(root_config_directory)
    else:  # Unix and GNU/Linux
        jd_develop_directory = os.path.expanduser("~/.config/jd-develop/")
        root_config_directory = os.path.abspath(os.path.expanduser("~/.config/jd-develop/geodesie/"))
        if not os.path.isdir(jd_develop_directory):
            os.mkdir(jd_develop_directory)
        if not os.path.isdir(root_config_directory):
            os.mkdir(root_config_directory)

    return root_config_directory + "/"


def create_save(save_name: str):
    config_dir = _determine_config_directory()
    full_save_dir = config_dir + save_name
    if not os.path.isdir(full_save_dir):
        os.mkdir(full_save_dir)

