// SPDX-License-Identifier: MIT You, 2022-03-09 - initial commit
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use grammers_client::types::{CallbackQuery, InlineQuery, Message, User};
// use crate::locales::Locale;
// use crate::database::Database;

#[derive(Debug, Default)]
pub struct Data {
    pub query: String,
    pub callback: Option<CallbackQuery>,
    pub inline: Option<InlineQuery>,
    pub message: Option<Message>,
    // pub language: Option<Locale>,
    // pub database: Option<Database>,
    pub me: Option<User>,
}
