// Copyright (C) 2022 Lílian Ferreira de Freitas
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
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
use wasm_bindgen::prelude::*;

static ALLOWED: &'static [char] = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

#[wasm_bindgen]
pub fn is_valid(document_string: &str) -> bool {
    let mut document: Vec<u32> = Vec::new();
    for c in document_string.chars() {
        if ALLOWED.contains(&c) {
            document.push(c.to_digit(10).unwrap_or(0));
        }
    }

    match document.len() {
        11 => {
            if document.iter().all(|c| document[0] == *c) {
                false // all numbers are the same
            } else {
                let mut k = (2..=10).rev();
                let sum: u32 = document.iter()
                    .map(|n| n * k.next().unwrap_or(0))
                    .sum();

                let first_digit = (sum * 10) % 11 % 10;

                let mut k = (2..=11).rev();
                let sum: u32 = document.iter()
                    .map(|n| n * k.next().unwrap_or(0))
                    .sum();

                let second_digit = (sum * 10) % 11 % 10;

                if &[first_digit, second_digit] == &document[9..=10] {
                    true
                } else {
                    false
                }
            }
        },
        14 => {
            if document.iter().all(|c| document[0] == *c) {
                false // all numbers are the same
            } else {
                let mut k: Vec<u32> = Vec::new();
                k.extend((2..=5).rev());
                k.extend((2..=9).rev());
                let mut k = k.into_iter();
                let sum: u32 = document.iter()
                    .map(|n| n * k.next().unwrap_or(0))
                    .sum();

                let first_digit = if (sum * 10) % 11 < 2 {
                    0
                } else {
                    11 - (sum % 11)
                };


                let mut k: Vec<u32> = Vec::new();
                k.extend((2..=6).rev());
                k.extend((2..=9).rev());
                let mut k = k.into_iter();
                let sum: u32 = document.iter()
                    .map(|n| n * k.next().unwrap_or(0))
                    .sum();

                let second_digit = if (sum * 10) % 11 < 2 {
                    0
                } else {
                    11 - (sum % 11)
                };

                if &[first_digit, second_digit] == &document[12..=13] {
                    true
                } else {
                    false
                }
            }
        },
        _ => false,
    }
}
