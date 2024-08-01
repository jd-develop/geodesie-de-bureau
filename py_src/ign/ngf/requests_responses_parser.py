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
from html.parser import HTMLParser


class SearchParser(HTMLParser):
    """Parses the answer of the RN search"""
    def __init__(self):
        super().__init__()
        self.rn_list: list[tuple[int, str]] = []
        self.currently_in_li = False

    def handle_starttag(self, tag: str, attrs: list[tuple[str, str | None]]):
        if tag == "li":
            self.currently_in_li = True
            if attrs[0][1] and attrs[0][1] != "result":
                self.rn_list.append((int(attrs[0][1]), ""))

    def handle_endtag(self, tag: str):
        if tag == "li":
            self.currently_in_li = False

    def handle_data(self, data: str):
        if self.currently_in_li and len(self.rn_list) > 0 and \
                self.rn_list[-1][1] == "":
            cid, _ = self.rn_list.pop()
            self.rn_list.append((cid, data))

