// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use crate::locales::Language;

#[derive(Debug, Default, Copy, Clone)]
pub struct Locale {
    language: Language,
}

impl Locale {
    pub fn get(&self, key: &str) -> String {
        rust_i18n::t!(key, locale = self.code())
    }

    pub fn get_with_code(key: &str, code: &str) -> String {
        rust_i18n::t!(key, locale = code)
    }

    pub fn code(&self) -> &str {
        self.language.code()
    }

    pub fn language(&self) -> Language {
        self.language
    }
}

impl From<String> for Locale {
    fn from(code: String) -> Self {
        Self {
            language: Language::from(code.as_str()),
        }
    }
}

impl From<&str> for Locale {
    fn from(code: &str) -> Self {
        Self {
            language: Language::from(code),
        }
    }
}
