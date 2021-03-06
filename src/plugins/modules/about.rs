// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use dyn_fmt::AsStrFormatExt;
use grammers_client::{types, Client, InputMessage};

use crate::locales::Locale;
use crate::plugins::{Data, Handler, HandlerType, Plugin, Result};
use crate::utils;

fn default_input(locale: Locale, me: types::User, is_private: bool) -> InputMessage {
    let mut input = InputMessage::html(locale.get("plugins.about.about").format(&[
        me.first_name(),
        &utils::make_html_url("https://github.com/AndrielFR/AmimeWatch", "GitHub"),
        &utils::make_html_url(
            utils::channel_url(locale.language()),
            &locale.get("words.channel"),
        ),
        &utils::make_html_url(
            utils::group_url(locale.language()),
            &locale.get("words.group"),
        ),
    ]));

    if is_private {
        let markup = utils::make_keyboard(vec![&[(
            locale.get("buttons.back"),
            "start".to_string(),
            "inline".to_string(),
        )]]);
        input = input.reply_markup(&markup);
    }

    input
}

#[macro_rules_attribute(crate::dyn_async)]
async fn about_message(_client: Client, data: Data) -> Result {
    let message = data.message.unwrap();
    let locale = data.locale.unwrap();
    let me = data.me.unwrap();

    if let types::Chat::Group(_) = message.chat() {
        message.reply(default_input(locale, me, false)).await?;

        return Ok(());
    }

    message.reply(default_input(locale, me, true)).await?;

    Ok(())
}

#[macro_rules_attribute(crate::dyn_async)]
async fn about_callback(_client: Client, data: Data) -> Result {
    let mut callback = data.callback.unwrap();
    let locale = data.locale.unwrap();
    let me = data.me.unwrap();

    callback
        .answer()
        .edit(default_input(locale, me, true))
        .await?;

    Ok(())
}

pub fn module() -> Plugin {
    Plugin::new("about")
        .register(
            Handler::new("about$", HandlerType::Message)
                .set_is_regex(true)
                .set_is_command(true)
                .set_use_i18n(true)
                .set_function(about_message),
        )
        .register(
            Handler::new("about$", HandlerType::CallbackQuery)
                .set_is_regex(true)
                .set_use_i18n(true)
                .set_function(about_callback),
        )
}
