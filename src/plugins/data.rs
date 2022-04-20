// SPDX-License-Identifier: MIT You, 2022-03-09 - initial commit
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use grammers_client::types::{CallbackQuery, InlineQuery, Message, User};
use sqlx::MySqlPool;

use crate::locales::Locale;

#[derive(Debug, Default)]
pub struct Data {
    pub query: String,
    pub callback: Option<CallbackQuery>,
    pub inline: Option<InlineQuery>,
    pub message: Option<Message>,
    pub locale: Option<Locale>,
    pub database: Option<MySqlPool>,
    pub me: Option<User>,
}
