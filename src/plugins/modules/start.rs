// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use dyn_fmt::AsStrFormatExt;
use grammers_client::{types, Client, InputMessage};

use crate::locales::Locale;
use crate::plugins::{Data, Handler, HandlerType, Plugin, Result};
use crate::utils;

fn default_input(locale: Locale, user: &types::Chat, me: types::User) -> InputMessage {
    let markup = utils::make_keyboard(vec![&[
        (
            locale.get("buttons.language"),
            "language".to_string(),
            "inline".to_string(),
        ),
        (
            locale.get("buttons.about"),
            "about".to_string(),
            "inline".to_string(),
        ),
    ]]);
    InputMessage::html(
        locale
            .get("plugins.start.start")
            .format(&[&utils::make_html_mention(user), me.first_name()]),
    )
    .reply_markup(&markup)
}

#[macro_rules_attribute(crate::dyn_async)]
async fn start_message(_client: Client, data: Data) -> Result {
    let message = data.message.unwrap();
    let locale = data.locale.unwrap();
    let user = message.sender().unwrap();
    let me = data.me.unwrap();

    if let types::Chat::Group(_) = message.chat() {
        let markup = utils::make_keyboard(vec![&[(
            locale.get("buttons.pm"),
            "https://t.me/{}/?start".format(&[me.username().unwrap()]),
            "url".to_string(),
        )]]);
        message
            .reply(
                InputMessage::html(
                    locale
                        .get("plugins.start.start_group")
                        .format(&[&utils::make_html_mention(&user), me.first_name()]),
                )
                .reply_markup(&markup),
            )
            .await?;

        return Ok(());
    }

    message.reply(default_input(locale, &user, me)).await?;

    Ok(())
}

#[macro_rules_attribute(crate::dyn_async)]
async fn start_callback(_client: Client, data: Data) -> Result {
    let mut callback = data.callback.unwrap();
    let locale = data.locale.unwrap();
    let me = data.me.unwrap();

    let input = {
        let user = callback.sender();
        default_input(locale, user, me)
    };

    callback.answer().edit(input).await?;

    Ok(())
}

pub fn module() -> Plugin {
    Plugin::new("start")
        .register(
            Handler::new("start$", HandlerType::Message)
                .set_is_regex(true)
                .set_is_command(true)
                .set_use_i18n(true)
                .set_function(start_message),
        )
        .register(
            Handler::new("start$", HandlerType::CallbackQuery)
                .set_is_regex(true)
                .set_use_i18n(true)
                .set_function(start_callback),
        )
}
