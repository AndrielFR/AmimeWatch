// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use grammers_client::types::{button, reply_markup};

pub fn make_keyboard(buttons: Vec<&[(&str, &str, &str)]>) -> reply_markup::Inline {
    let mut rows = Vec::new();

    for line in buttons {
        let mut row = Vec::with_capacity(line.len());

        for item in line {
            match item.2 {
                "url" => row.push(button::url(item.0, item.1)),
                _ => row.push(button::inline(item.0, item.1.as_bytes())),
            }
        }

        rows.push(row);
    }

    reply_markup::inline(rows)
}
