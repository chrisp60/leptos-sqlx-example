//! A module with items that can be used by the server, and the client.
//!
//! Some people combine these things into the app.rs file, I prefer to keep
//! Components and views seperate from structural business logic.

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

// Only derive sqlx::FromRow when ssr is enabled
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
/// A users account and their favorite food.
#[derive(Debug, Clone, Serialize, Deserialize, bon::Builder)]
pub struct Animal {
    pub name: String,
    pub created: chrono::DateTime<chrono::Utc>,
    pub score: i64,
    pub nickname: Option<String>,
}

impl Animal {
    /// Forwards to [`search_animals`].
    ///
    /// Forwarding to server functions is a nice way to maintain a clean api.
    pub async fn search(name: Option<String>) -> Result<Vec<Self>, ServerFnError> {
        search_animal(name).await
    }
}

#[server]
pub async fn create_animal(
    name: String,
    score: Option<i64>,
    nickname: Option<String>,
) -> Result<Option<Animal>, ServerFnError> {
    println!("Creating an animal named: {name}");
    let insert = sqlx::query_as!(
        Animal,
        r#"
        insert into animal (
            name,
            score,
            nickname
        ) values ( ?1, ?2, ?3 )
        on conflict do nothing
        returning
            name,
            created as "created: _", 
            score,
            nickname
        "#,
        name,
        score,
        nickname
    )
    .fetch_optional(crate::server::use_pool())
    .await?;
    Ok(insert)
}

/// Searches for Some(name) or returns everything when None.
#[server]
pub async fn search_animal(name: Option<String>) -> Result<Vec<Animal>, ServerFnError> {
    use crate::server::use_pool;
    println!("Searching for: {name:?}");

    let results = sqlx::query_as!(
        Animal,
        r#"
        select
            name,
            -- using special syntax to keep compile-time checks.
            -- https://docs.rs/sqlx/latest/sqlx/macro.query_as.html#column-type-override-infer-from-struct-field
            created as "created: _", 
            score,
            nickname
        from 
            animal
        where
            -- A trick for optional query parameters
            (?1 is null) or (name = ?1)
        "#,
        name
    )
    .fetch_all(use_pool()).await?;
    println!("Returning: {results:#?}");
    Ok(results)
}
