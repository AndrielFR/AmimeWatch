// SPDX-License-Identifier: MIT You, 2022-03-09 - initial commit
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

#[derive(Debug, ormx::Table)]
#[ormx(table = "users", id = id, insertable)]
pub struct User {
    #[ormx(column = "id")]
    #[ormx(get_one = get_by_id(i64))]
    #[ormx(get_optional(i64))]
    pub id: u32,
    #[ormx(get_optional(&str))]
    pub language: String,
}
