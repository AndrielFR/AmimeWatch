// SPDX-License-Identifier: MIT You, 2022-03-09 - initial commit
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use grammers_client::{Client, Update};

use crate::plugins::modules::start;
use crate::plugins::{Data, Plugin, Result};

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

            match update {
                Update::CallbackQuery(ref callback) => {
                    query = std::str::from_utf8(callback.data()).unwrap().to_string();
                }
                Update::InlineQuery(ref inline) => {
                    query = inline.text().to_string();
                }
                Update::NewMessage(ref message) => {
                    query = message.text().to_string();
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

                        let mut data = Data {
                            query,
                            me: Some(me.clone()),
                            ..Default::default()
                        };

                        match update {
                            Update::CallbackQuery(callback) => {
                                data.callback = Some(callback);
                            }
                            Update::InlineQuery(inline) => {
                                data.inline = Some(inline);
                            }
                            Update::NewMessage(message) => {
                                data.message = Some(message);
                            }
                            _ => {}
                        }

                        handler.run(client.clone(), data).await?;
                        break;
                    }
                }
            }
        }

        Ok(())
    }

    pub fn add_plugin(mut self, plugin: Plugin) -> Self {
        self.plugins.push(plugin);
        self
    }

    pub fn load_plugins(self) -> Self {
        self.add_plugin(start::module())
    }

    pub fn prefixes(&self) -> &Vec<String> {
        &self.prefixes
    }
}
