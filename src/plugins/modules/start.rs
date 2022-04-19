// SPDX-License-Identifier: MIT You, 2022-03-09 - initial commit
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use grammers_client::{Client, InputMessage};

use crate::plugins::{Data, Handler, HandlerType, Plugin, Result};

#[macro_rules_attribute(crate::dyn_async)]
async fn start(_client: Client, data: Data) -> Result {
    let message = data.message.unwrap();
    let locale = data.locale.unwrap();

    message.reply(InputMessage::html(locale.get("NAME"))).await?;

    Ok(())
}

pub fn module() -> Plugin {
    Plugin::default().register(
        Handler::new("start$")
            .set_type(HandlerType::Message)
            .set_is_regex(true)
            .set_is_command(true)
            .set_use_i18n(true)
            .set_function(start),
    )
}
