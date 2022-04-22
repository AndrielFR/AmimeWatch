// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use grammers_client::types::{CallbackQuery, InlineQuery, Message, User};
use sqlx::MySqlPool;

use crate::locales::Locale;
use crate::plugins::HandlerType;

#[derive(Debug, Default)]
pub struct Data {
    pub query: String,
    pub user_id: i64,
    pub group_id: i64,
    pub callback: Option<CallbackQuery>,
    pub inline: Option<InlineQuery>,
    pub message: Option<Message>,
    pub locale: Option<Locale>,
    pub database: Option<MySqlPool>,
    pub me: Option<User>,
    pub update_type: HandlerType,
}
