// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use grammers_client::types::{button, reply_markup, Chat};

use crate::locales::Language;

pub fn make_keyboard(buttons: Vec<&[(String, String, String)]>) -> reply_markup::Inline {
    let mut rows = Vec::with_capacity(buttons.len());

    for line in buttons {
        let mut row = Vec::with_capacity(line.len());

        for item in line {
            match item.2.as_str() {
                "url" => row.push(button::url(&item.0, &item.1)),
                _ => row.push(button::inline(&item.0, item.1.as_bytes())),
            }
        }

        rows.push(row);
    }

    reply_markup::inline(rows)
}

pub fn make_html_mention(chat: &Chat) -> String {
    match chat {
        Chat::User(user) => {
            format!(
                "<a href=\"tg://user?id={}\">{}</a>",
                user.id(),
                user.first_name()
            )
        }
        c => {
            format!("<a href=\"https://t.me/c/{}/-1\">{}</a>", c.id(), c.name())
        }
    }
}

pub fn make_html_url(url: &str, text: &str) -> String {
    format!("<a href=\"{}\">{}</a>", url, text)
}

pub fn channel_url<'a>(language: Language) -> &'a str {
    match language {
        Language::English => "https://t.me/AmimeWatch",
        _ => "https://t.me/AmimeWatchChannelPT",
    }
}

pub fn group_url<'a>(language: Language) -> &'a str {
    match language {
        Language::English => "https://t.me/AmimeWatchGroup",
        _ => "https://t.me/AmimeWatchGroupPT",
    }
}
