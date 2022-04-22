// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use grammers_client::{reply_markup, types, Client, InputMessage};

use crate::database::tables;
use crate::locales::{Language, Locale};
use crate::plugins::{Data, Handler, HandlerLevel, HandlerType, Plugin, Result};
use crate::utils;

fn default_input(locale: Locale, is_private: bool) -> InputMessage {
    let markup = default_keyboard(locale, is_private);
    InputMessage::html(locale.get("plugins.language.language")).reply_markup(&markup)
}

fn default_keyboard(locale: Locale, include_back_button: bool) -> reply_markup::Inline {
    let mut buttons: Vec<&[(String, String, String)]> = Vec::new();

    let mut text;
    let mut data;

    let mut language_buttons: Vec<(String, String, String)> = Vec::new();
    for language in Language::all().iter() {
        if language == &locale.language() {
            text = format!("{} {} 📌", locale.get("FLAG"), locale.get("NAME"));
            data = format!("language {}", locale.code());
        } else {
            text = format!(
                "{} {}",
                Locale::get_with_code("FLAG", language.code()),
                Locale::get_with_code("NAME", language.code())
            );
            data = format!("language {}", language.code());
        }

        language_buttons.push((text, data, "inline".to_string()));
    }

    for index in 0..language_buttons.len() {
        if index % 2 == 0 {
            buttons.push(&language_buttons[index..index + 2]);
        }
    }

    let back_button = vec![(
        locale.get("buttons.back"),
        "start".to_string(),
        "inline".to_string(),
    )];
    if include_back_button {
        buttons.push(back_button.as_slice());
    }

    utils::make_keyboard(buttons)
}

#[macro_rules_attribute(crate::dyn_async)]
async fn language_message(_client: Client, data: Data) -> Result {
    let message = data.message.unwrap();
    let locale = data.locale.unwrap();

    if let types::Chat::Group(_) = message.chat() {
        message.reply(default_input(locale, false)).await?;

        return Ok(());
    }

    message.reply(default_input(locale, true)).await?;

    Ok(())
}

#[macro_rules_attribute(crate::dyn_async)]
async fn language_callback(_client: Client, data: Data) -> Result {
    let mut callback = data.callback.unwrap();
    let locale = data.locale.unwrap();

    if let types::Chat::Group(_) = callback.chat() {
        callback.answer().edit(default_input(locale, false)).await?;

        return Ok(());
    }

    callback.answer().edit(default_input(locale, true)).await?;

    Ok(())
}

#[macro_rules_attribute(crate::dyn_async)]
async fn language_code_callback(client: Client, mut data: Data) -> Result {
    let query = data.query.clone();
    let group_id = data.group_id;
    let user_id = data.user_id;
    let locale = data.locale.unwrap();
    let database = data.database.clone().unwrap();

    let query_splitted = query.split_ascii_whitespace().collect::<Vec<&str>>();
    let new_code = query_splitted.last().unwrap();
    if new_code == &locale.code() {
        return Ok(());
    }

    if group_id == user_id {
        let mut user_db = tables::User::get_by_id(&database, user_id).await?;
        user_db
            .set_language(&database, new_code.to_string())
            .await?;
    } else {
        let mut group_db = tables::Group::get_by_id(&database, group_id).await?;
        group_db
            .set_language(&database, new_code.to_string())
            .await?;
    }

    data.locale = Some(Locale::from(*new_code));

    language_callback(client, data).await?;

    Ok(())
}

pub fn module() -> Plugin {
    Plugin::new("language")
        .register(
            Handler::new("language$", HandlerType::Message)
                .set_is_regex(true)
                .set_is_command(true)
                .set_use_i18n(true)
                .set_function(language_message)
                .set_level(HandlerLevel::Administrator),
        )
        .register(
            Handler::new("language$", HandlerType::CallbackQuery)
                .set_is_regex(true)
                .set_use_i18n(true)
                .set_function(language_callback)
                .set_level(HandlerLevel::Administrator),
        )
        .register(
            Handler::new(r"language (\w+)", HandlerType::CallbackQuery)
                .set_is_regex(true)
                .set_use_i18n(true)
                .set_use_database(true)
                .set_function(language_code_callback)
                .set_level(HandlerLevel::Administrator),
        )
}
