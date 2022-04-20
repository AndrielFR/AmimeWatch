// SPDX-License-Identifier: MIT You, 2022-03-09 - initial commit
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use grammers_client::{Client, Update};
use sqlx::MySqlPool;
use ormx::Insert;

use crate::plugins::modules::start;
use crate::plugins::{Data, HandlerType, Plugin, Result};
use crate::locales::Locale;
use crate::database::tables;

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

    pub async fn run(self, mut client: Client, database_url: String) -> Result {
        log::info!("running");
        let me = client.get_me().await?;

        rust_i18n::set_locale("pt");

        while let Some(update) = tokio::select! {
            _ = tokio::signal::ctrl_c() => Ok(None),
            result = client.next_update() => result,
        }? {
            let mut group_id = 0;
            let mut user_id = 0;
            let mut query = String::new();

            let mut data = Data {
                query: query.clone(),
                me: Some(me.clone()),
                update_type: HandlerType::default(),
                ..Default::default()
            };

            match update {
                Update::CallbackQuery(callback) => {
                    group_id = callback.chat().id();
                    user_id = callback.sender().id();
                    query = std::str::from_utf8(callback.data()).unwrap().to_string();
                    data.callback = Some(callback);
                    data.update_type = HandlerType::CallbackQuery;
                }
                Update::InlineQuery(inline) => {
                    user_id = inline.sender().id();
                    group_id = user_id;
                    query = inline.text().to_string();
                    data.inline = Some(inline);
                    data.update_type = HandlerType::InlineQuery;
                }
                Update::NewMessage(message) => {
                    group_id = message.chat().id();
                    user_id = message.sender().unwrap().id();
                    query = message.text().to_string();
                    data.message = Some(message);
                    data.update_type = HandlerType::Message;
                }
                _ => {}
            }

            for plugin in self.plugins() {
                match plugin.check(&query, me.username().unwrap(), self.prefixes()) {
                    -1 => continue,
                    id => {
                        let handler = plugin.get_handler(id);
                        let database = MySqlPool::connect(&database_url).await?;

                        if handler.use_i18n() {
                            let locale = if group_id == user_id {
                                match tables::User::by_id(&database, user_id).await? {
                                   Some(user) => Locale::from(user.language),
                                   None => {
                                       let locale = Locale::default();
                                       tables::InsertUser {
                                           id: user_id as u32,
                                           language: locale.code().to_string(),
                                       }
                                       .insert(&mut *database.acquire().await?)
                                       .await?;

                                       locale
                                   },
                                }
                            } else {
                                match tables::Group::by_id(&database, group_id).await? {
                                   Some(group) => Locale::from(group.language),
                                   None => {
                                       let locale = Locale::default();
                                       tables::InsertGroup {
                                           id: group_id as i32,
                                           language: locale.code().to_string(),
                                       }
                                       .insert(&mut *database.acquire().await?)
                                       .await?;

                                       locale
                                   },
                                }
                            };

                            data.locale = Some(locale);
                        }

                        if handler.use_database() {
                            data.database = Some(database);
                        }

                        if handler.r#type() == &data.update_type {
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
