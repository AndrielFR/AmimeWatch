// SPDX-License-Identifier: MIT You, 2022-03-09 - initial commit
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

#![allow(dead_code)]
#![allow(unused_macros)]

mod config;
pub mod plugins;

pub use config::Config;

#[macro_use]
extern crate macro_rules_attribute;

#[macro_use]
extern crate serde_derive;

#[macro_export]
macro_rules! dyn_async {(
    $( #[$attr:meta] )*
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
