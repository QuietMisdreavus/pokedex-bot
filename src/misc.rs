// pokedex-bot - an irc bot that relays information about pokemon
// Copyright (C) 2016 Bryan Mitchell (icesoldier)
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

pub fn starts_with(s1: &str, s2: &str) -> bool {
    if s1.len() < s2.len() {
        return false;
    }
    for (c1, c2) in s1.chars().zip(s2.chars()) {
        if c1 != c2 {
            return false;
        }
    }
    true
}
