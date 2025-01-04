# Leptos & Sqlx Example

A simple example that shows how to use Sqlx with leptos 0.7

## Serving

- You will need sqlite installed

```shell
sqlx db create # create the sqlite db
slqx migrate run # run migrations
cargo leptos serve # serve the app
```

## Caveats

- This example is minimal, and does no effort to be efficient, or protect against errors.
