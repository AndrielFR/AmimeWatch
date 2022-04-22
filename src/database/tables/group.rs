// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

#[derive(Debug, ormx::Table)]
#[ormx(table = "groups", id = id, insertable)]
pub struct Group {
    #[ormx(column = "id")]
    #[ormx(get_one = get_by_id(i64))]
    #[ormx(get_optional(i64))]
    pub id: i32,
    #[ormx(get_optional(&str), set)]
    pub language: String,
}
