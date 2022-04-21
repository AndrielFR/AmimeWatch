// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

#![allow(dead_code)]
#![allow(unused_macros)]

mod config;
pub mod database;
pub mod locales;
pub mod plugins;
pub mod utils;

pub use config::Config;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate macro_rules_attribute;

// Load the languages
rust_i18n::i18n!("./src/locales/files/");

#[macro_export]
macro_rules! dyn_async {(
    $(#[$attr:meta])*
    $pub:vis
    async
    fn $fname:ident ($($args:tt)*) $(-> $Ret:ty)? {
        $($body:tt)*
    }
) => (
    $(#[$attr])*
    #[allow(unused_parens)]
    $pub
    fn $fname ($($args)*) -> ::std::pin::Pin<::std::boxed::Box<
        dyn ::std::future::Future<Output = ($($Ret)?)>
            + ::std::marker::Send + 'static
    >> {
        ::std::boxed::Box::pin(async move { $($body)* })
    }
)}
