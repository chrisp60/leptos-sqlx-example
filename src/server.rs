use std::sync::LazyLock;

use axum::Router;
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use sqlx::SqlitePool;

use crate::shared;

#[tokio::main]
pub async fn run() {
    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(crate::app::App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || crate::app::shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(crate::app::shell))
        .with_state(leptos_options);

    axum::serve(
        tokio::net::TcpListener::bind(&addr).await.unwrap(),
        app.into_make_service(),
    )
    .await
    .unwrap();
}

/// A global function that can be called on the server to use our database pool.
///
/// You can also put this in Axum's State: https://docs.rs/axum/latest/axum/extract/struct.State.html#with-router
pub fn use_pool() -> &'static SqlitePool {
    // WARN: There are downsides to using a static LazyLock like this, but for
    // productivity, it is quick and easy.
    static POOL: LazyLock<SqlitePool> = LazyLock::new(|| {
        let database_url = dotenvy::var("DATABASE_URL").expect("database url to be set");
        SqlitePool::connect_lazy(&database_url).expect("database connection")
    });
    &POOL
}
