fn main() {
    #[cfg(feature = "ssr")]
    leptos_sqlx_example::server::run()
}
