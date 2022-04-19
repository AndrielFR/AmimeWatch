// SPDX-License-Identifier: MIT You, 2022-03-09 - initial commit
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use crate::locales::Language;

// TODO: Remove default()
#[derive(Debug, Default)]
pub struct Locale {
    language: Language,
}

impl Locale {
    pub fn get(&self, key: &str) -> String {
        rust_i18n::t!(key, locale = self.language.code())
    }
}

impl From<&str> for Locale {
    fn from(code: &str) -> Self {
        Self {
            language: Language::from(code),
        }
    }
}
