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
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/demo_store.css"/>

        // sets the document title
        <Title text="Welcome to the shop"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=StaticSegment("/store") view=StorePage/>
                    <Route path=StaticSegment("/control_panel") view=ControlPage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <h1>"Welcome to the shop!"</h1>
        <nav>
            <a href="/store">"Store"</a>
            <a href="/control_panel">"Control Panel"</a>
        </nav>
    }
}

#[component]
fn StorePage() -> impl IntoView {
    view!{
        <h1>"Store"</h1>
        <nav>
            <a href="/store">"Store"</a>
            <a href="/control_panel">"Control Panel"</a>
        </nav>
    }
}

#[component]
fn ControlPage() -> impl IntoView {
    view!{
        <h1>"Control Panel"</h1>
        <nav>
            <a href="/store">"Store"</a>
            <a href="/control_panel">"Control Panel"</a>
        </nav>
    }
}