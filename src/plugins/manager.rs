// SPDX-License-Identifier: MIT You, 2022-03-09 - initial commit
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use grammers_client::{Client, Update};

use crate::plugins::modules::start;
use crate::plugins::{Data, HandlerType, Plugin, Result};
use crate::locales::Locale;

pub struct Manager {
    prefixes: Vec<String>,
    plugins: Vec<Plugin>,
}

impl Manager {
    pub fn new(prefixes: Vec<String>) -> Self {
        Self {
            prefixes,
            plugins: Vec::new(),
        }
    }

    pub async fn run(self, mut client: Client) -> Result {
        log::info!("running");
        let me = client.get_me().await?;

        rust_i18n::set_locale("pt");

        while let Some(update) = tokio::select! {
            _ = tokio::signal::ctrl_c() => Ok(None),
            result = client.next_update() => result,
        }? {
            let user_id;
            let mut query = String::new();
            let mut update_type = HandlerType::default();

            let mut data = Data {
                query: query.clone(),
                me: Some(me.clone()),
                ..Default::default()
            };

            match update {
                Update::CallbackQuery(callback) => {
                    user_id = callback.sender().id();
                    query = std::str::from_utf8(callback.data()).unwrap().to_string();
                    data.callback = Some(callback);
                    update_type = HandlerType::CallbackQuery;
                }
                Update::InlineQuery(inline) => {
                    user_id = inline.sender().id();
                    query = inline.text().to_string();
                    data.inline = Some(inline);
                    update_type = HandlerType::InlineQuery;
                }
                Update::NewMessage(message) => {
                    user_id = message.sender().unwrap().id();
                    query = message.text().to_string();
                    data.message = Some(message);
                    update_type = HandlerType::Message;
                }
                _ => {}
            }

            for plugin in self.plugins() {
                match plugin.check(&query, me.username().unwrap(), self.prefixes()) {
                    -1 => continue,
                    id => {
                        let handler = plugin.get_handler(id);

                        if handler.use_i18n() {
                            let locale_code = "pt";
                            let locale = Locale::from(locale_code);
                            data.locale = Some(locale);
                        }

                        if handler.r#type() == &update_type {
                            match handler.run(client.clone(), data).await {
                                Ok(_) => {},
                                Err(e) => log::error!("an error ocurred while handling: {}", e),
                            }
                            break;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn add_plugin(mut self, plugin: Plugin) -> Self {
        self.plugins.push(plugin);
        self
    }

    pub fn load_plugins(self) -> Self {
        log::info!("loading plugins...");
        self.add_plugin(start::module())
    }

    fn prefixes(&self) -> &[String] {
        &self.prefixes
    }

    fn plugins(&self) -> &[Plugin] {
        &self.plugins
    }
}
