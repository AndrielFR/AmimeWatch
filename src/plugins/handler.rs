// SPDX-License-Identifier: MIT You, 2022-03-09 - initial commit
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use std::{future::Future, pin::Pin};

use grammers_client::Client;
use regex::Regex;

use crate::plugins::Data;

pub type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub type AsyncFunction = fn(Client, Data) -> Pin<Box<dyn Future<Output = Result> + Send + 'static>>;

#[macro_rules_attribute(crate::dyn_async)]
async fn not_made_yet(_client: Client, _data: Data) -> Result {
    unimplemented!()
}

pub struct Handler {
    pattern: &'static str,
    function: AsyncFunction,
    r#type: Type,
    is_regex: bool,
    is_command: bool,
    use_i18n: bool,
    use_database: bool,
}

impl Handler {
    pub fn new(pattern: &'static str) -> Self {
        Self {
            pattern,
            function: not_made_yet,
            r#type: Type::default(),
            is_regex: false,
            is_command: true,
            use_i18n: false,
            use_database: false,
        }
    }

    pub async fn run(&self, client: Client, data: Data) -> Result {
        self.function()(client, data).await?;
        Ok(())
    }

    pub fn check(&self, query: &str, username: &str, prefixes: &[String]) -> bool {
        let mut pattern = self.pattern().to_string();
        let mut has_final_line_symbol = false;

        if self.is_regex() && pattern.ends_with('$') {
            pattern.pop();
            has_final_line_symbol = true;
        }

        if self.is_command() {
            let pattern_clone = pattern.clone();
            let pattern_splitted = pattern_clone
                .split_ascii_whitespace()
                .collect::<Vec<&str>>();

            if pattern_splitted.len() > 1 {
                pattern.clear();
                pattern.push_str(&pattern_splitted[..1].join(" "));
            }

            pattern.push_str(format!("(?:@{})?", username).as_str());
            pattern.insert_str(0, format!("^[{}]", prefixes.join("")).as_str());

            let pattern_parts = &pattern_splitted[1..];
            for pattern_part in pattern_parts {
                pattern.push_str(format!(" {}", pattern_part).as_str());
            }
        }

        if self.is_regex() && has_final_line_symbol {
            pattern.push('$');
        }

        let pattern = pattern.as_str();

        let re = Regex::new(pattern).unwrap();
        if re.is_match(query) {
            return true;
        }

        false
    }

    fn function(&self) -> AsyncFunction {
        self.function
    }

    pub fn r#type(&self) -> &Type {
        &self.r#type
    }

    fn pattern(&self) -> &str {
        self.pattern
    }

    fn is_regex(&self) -> bool {
        self.is_regex
    }

    fn is_command(&self) -> bool {
        self.is_command
    }

    pub fn use_i18n(&self) -> bool {
        self.use_i18n
    }

    pub fn use_database(&self) -> bool {
        self.use_database
    }

    pub fn set_function(mut self, function: AsyncFunction) -> Self {
        self.function = function;
        self
    }

    pub fn set_type(mut self, value: Type) -> Self {
        self.r#type = value;
        self
    }

    pub fn set_is_regex(mut self, value: bool) -> Self {
        self.is_regex = value;
        self
    }

    pub fn set_is_command(mut self, value: bool) -> Self {
        self.is_command = value;
        self
    }

    pub fn set_use_i18n(mut self, value: bool) -> Self {
        self.use_i18n = value;
        self
    }

    pub fn set_use_database(mut self, value: bool) -> Self {
        self.use_database = value;
        self
    }
}

#[derive(Debug, PartialEq)]
pub enum Type {
    CallbackQuery,
    InlineQuery,
    Message,
}

impl Type {
    pub fn as_str(&self) -> &str {
        match self {
            Self::CallbackQuery => "CallbackQuery",
            Self::InlineQuery => "InlineQuery",
            Self::Message => "Message",
        }
    }
}

impl Default for Type {
    fn default() -> Self {
        Self::Message
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str().to_ascii_lowercase())
    }
}
