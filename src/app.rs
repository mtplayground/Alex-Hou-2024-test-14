use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
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
        <Stylesheet href="/vendor/todomvc-common/base.css"/>
        <Stylesheet href="/vendor/todomvc-app-css/index.css"/>
        <Stylesheet id="leptos" href="/pkg/alex-hou-2024-test-14.css"/>
        <Title text="TodoMVC"/>
        <Router>
            <main class="todoapp-shell">
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=TodoApp/>
                    <Route path=StaticSegment("active") view=TodoApp/>
                    <Route path=StaticSegment("completed") view=TodoApp/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn TodoApp() -> impl IntoView {
    view! {
        <>
            <section class="todoapp">
                <header class="header">
                    <h1>"todos"</h1>
                </header>
            </section>
            <footer class="info">
                <p>"Double-click to edit a todo"</p>
                <p>
                    "Created for "
                    <a href="https://todomvc.com">"TodoMVC"</a>
                </p>
            </footer>
        </>
    }
}
