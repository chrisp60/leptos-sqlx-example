/// Items that can be used on both the server, and client.
pub mod shared;

/// Components go here.
pub mod app;

/// A shared error module.
pub mod error;

// Mark the server module as only for ssr.
// Now everything inside the module does not need to be configured out.
#[cfg(feature = "ssr")]
pub mod server;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}

#[macro_export]
macro_rules! ssr {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "ssr")]
            #[allow(unused)]
            $item
        )*
    };
}

// You can use this macro to import a bunch of ssr stuff without repeating
// yourself.

crate::ssr! {
    use tokio::runtime::Runtime;
    use sqlx::SqlitePool;
    use crate::server::run;
}
