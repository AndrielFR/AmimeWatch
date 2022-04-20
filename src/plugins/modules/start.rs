// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use grammers_client::{types, Client, InputMessage};

use crate::plugins::{Data, Handler, HandlerType, Plugin, Result};
use crate::utils;

#[macro_rules_attribute(crate::dyn_async)]
async fn start_message(_client: Client, data: Data) -> Result {
    let message = data.message.unwrap();
    let locale = data.locale.unwrap();
    let user = message.sender().unwrap();
    let me = data.me.unwrap();

    let markup = utils::make_keyboard(vec![&[(&locale.get("buttons.about"), "about", "inline")]]);
    let input = InputMessage::html(
        locale
            .get("plugins.start.start")
            .format(&[user.name(), me.first_name()]),
    )
    .reply_markup(&markup);

    match message.chat() {
        types::Chat::Group(_) => {
            let markup = utils::make_keyboard(vec![&[(
                &locale.get("buttons.pm"),
                &"https://t.me/{}/?start".format(&[me.username().unwrap()]),
                "url",
            )]]);
            message
                .reply(
                    InputMessage::html(
                        locale
                            .get("plugins.start.start_group")
                            .format(&[user.name(), me.first_name()]),
                    )
                    .reply_markup(&markup),
                )
                .await?;

            return Ok(());
        }
        _ => {}
    }

    message.reply(input).await?;

    Ok(())
}

#[macro_rules_attribute(crate::dyn_async)]
async fn start_callback(_client: Client, data: Data) -> Result {
    let mut callback = data.callback.unwrap();
    let locale = data.locale.unwrap();
    let user = callback.sender();
    let me = data.me.unwrap();

    let markup = utils::make_keyboard(vec![&[(&locale.get("buttons.about"), "about", "inline")]]);
    let input = InputMessage::html(
        locale
            .get("plugins.start.start")
            .format(&[user.name(), me.first_name()]),
    )
    .reply_markup(&markup);

    callback.answer().edit(input).await?;

    Ok(())
}

pub fn module() -> Plugin {
    Plugin::default()
        .register(
            Handler::new("start$")
                .set_type(HandlerType::Message)
                .set_is_regex(true)
                .set_is_command(true)
                .set_use_i18n(true)
                .set_function(start_message),
        )
        .register(
            Handler::new("start$")
                .set_type(HandlerType::CallbackQuery)
                .set_is_regex(true)
                .set_use_i18n(true)
                .set_function(start_callback),
        )
}
