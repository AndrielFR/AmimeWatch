// SPDX-License-Identifier: MIT You, 2022-03-09 - initial commit
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use grammers_client::{Client, Update};

use crate::plugins::modules::start;
use crate::plugins::{Data, HandlerType, Plugin, Result};

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

    pub async fn run(&self, mut client: Client) -> Result {
        let me = client.get_me().await?;

        while let Some(update) = client.next_update().await? {
            let mut query = String::new();
            let mut update_type = HandlerType::default();

            let mut data = Data {
                query: query.clone(),
                me: Some(me.clone()),
                ..Default::default()
            };

            match update {
                Update::CallbackQuery(callback) => {
                    query = std::str::from_utf8(callback.data()).unwrap().to_string();
                    data.callback = Some(callback);
                    update_type = HandlerType::CallbackQuery;
                }
                Update::InlineQuery(inline) => {
                    query = inline.text().to_string();
                    data.inline = Some(inline);
                    update_type = HandlerType::InlineQuery;
                }
                Update::NewMessage(message) => {
                    query = message.text().to_string();
                    data.message = Some(message);
                    update_type = HandlerType::Message;
                }
                _ => {}
            }

            for plugin in self.plugins.iter() {
                match plugin
                    .check(&query, me.username().unwrap(), self.prefixes())
                    .await
                {
                    -1 => continue,
                    id => {
                        let handler = plugin.get_handler(id);

                        if handler.r#type() == &update_type {
                            handler.run(client.clone(), data).await?;
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
        self.add_plugin(start::module())
    }

    fn prefixes(&self) -> &[String] {
        &self.prefixes
    }
}
