// SPDX-License-Identifier: MIT You, 2022-03-09 - initial commit
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

#[derive(Debug)]
pub enum Language {
    Portuguese,
    English,
}

impl Language {
    pub fn code(&self) -> &str {
        match self {
            Self::English => "en",
            _ => "pt",
        }
    }
}

impl Default for Language {
    fn default() -> Self {
        Self::Portuguese
    }
}

impl From<&str> for Language {
    fn from(code: &str) -> Self {
        match code {
            "en" => Self::English,
            _ => Self::Portuguese,
        }
    }
}
