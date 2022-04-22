// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

#[derive(Debug, Copy, Clone, PartialEq)]
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

    pub fn all() -> Vec<Self> {
        vec![Self::Portuguese, Self::English]
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
