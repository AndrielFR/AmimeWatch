// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use crate::plugins::Handler;

pub struct Plugin {
    name: String,
    handlers: Vec<Handler>,
}

impl Plugin {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            handlers: Vec::new(),
        }
    }

    pub fn register(mut self, handler: Handler) -> Self {
        self.handlers.push(handler);
        self
    }

    pub fn check(&self, query: &str, username: &str, prefixes: &[String]) -> i16 {
        for (id, handler) in self.handlers.iter().enumerate() {
            match handler.check(query, username, prefixes) {
                true => return id.try_into().unwrap(),
                false => continue,
            }
        }

        -1
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn handlers(&self) -> &[Handler] {
        self.handlers.as_slice()
    }

    pub fn get_handler(&self, id: i16) -> &Handler {
        self.handlers().get(id as usize).unwrap()
    }

    pub fn set_name(mut self, value: &str) -> Self {
        self.name = value.to_string();
        self
    }
}
