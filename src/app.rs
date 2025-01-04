use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::shared::{Animal, CreateAnimal};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
      <!DOCTYPE html> 
      <html lang="en">
        <head>
          <meta charset="utf-8"/>
          <meta name="viewport" content="width=device-width, initial-scale=1"/>
          <HydrationScripts options/>
          <MetaTags/>
        </head>
        <body>
          <App/>
        </body>
      </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
      <Stylesheet id="leptos" href="/pkg/leptos-sqlx-example.css"/>
      <Title text="Welcome to Leptos"/>
      <Router>
        <main>
          <Routes fallback=|| "Page not found.".into_view()>
            <Route path=StaticSegment("") view=HomePage/>
          </Routes>
        </main>
      </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let create = ServerAction::<CreateAnimal>::new();
    let name = RwSignal::<String>::default();

    let results = Resource::new(
        // Include the create.version() in our deps so when we create a new animal, the results
        // will update.
        move || (create.version(), name.get()),
        |(_version, name)| {
            let name = Some(name).filter(|value| !value.is_empty());
            Animal::search(name)
        },
    );

    // This does not do any error handling. You should handle errors.
    let row = move || {
        results
            .and_then(Clone::clone)
            .transpose()
            .unwrap_or_default()
            .unwrap_or_default()
    };

    view! {
      <input
        type="search"
        required
        name="name"
        bind:value=name
        placeholder="Animal Name Search!"
      />
      <hr/>
      <ActionForm action=create>
        <input name="name" required placeholder="name"/>
        <input name="nickname" required placeholder="nickname"/>
        <input name="score" type="number" placeholder="score"/>
        <button type="submit">Create</button>
      </ActionForm>

      <Transition fallback=move || "Loading...">
        <table>
          {move || {
              row()
                  .into_iter()
                  .map(|animal| {
                      view! {
                        <tr>
                          <td>{animal.name}</td>
                          <td>{animal.nickname}</td>
                          <td>{animal.score}</td>
                          <td>{animal.created.format("%c").to_string()}</td>
                        </tr>
                      }
                  })
                  .collect_view()
          }}

        </table>
      </Transition>
    }
}
