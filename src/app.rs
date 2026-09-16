use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes, ParentRoute},
    path
};

use crate::storefront::product_cards;
use crate::control_panel::control_panel;

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
                    <Route path=path!("/") view=HomePage/>
                    <Route path=path!("/store") view=StorePage/>
                    // <ParentRoute path=path!("/control_panel") view=ControlPage>
                    //     <Route path=path!(":id") view=control_panel::ProductExpanded />
                    //     // <Route path=path!("/*any") view=|| view! { <h1>"Not Found"</h1> }/>
                    //     // <Route path=path!("") view=|| view! { <h1>"Not Found"</h1> }/>
                    // </ParentRoute>
                    <Route path=path!("/control_panel") view=ControlPage/>
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
        <AppNav/>
        // <control_panel::ProductControlView/>
    }
}

#[component]
fn StorePage() -> impl IntoView {
    view!{
        <h1>"Store"</h1>
        <AppNav/>
        <product_cards::ProductCardLayout/>
    }
}

#[component]
fn ControlPage() -> impl IntoView {
    view!{
        <h1>"Control Panel"</h1>
        <AppNav/>
        <control_panel::ProductControlView/>
    }
}

#[component]
fn AppNav() -> impl IntoView {
    let (value, set_value) = create_signal("/pkg/demo_store.css".to_string());

    view!{
        <nav id="app_nav">
            <a href="/store">"Store"</a>
            <a href="/control_panel">"Control Panel"</a>
        </nav>

        <select name="css_selection"
            prop:value=move || value.get()
            on:change:target= move |ev| {
                let new_value: String = ev.target().value(); //.value_of().to_js_string().as_string().unwrap_or_default();//.value_of();
                set_value.set(new_value);
            }
        >
            <option value="/pkg/demo_store.css">"main"</option>
            <option value="/test.css">"test"</option>
        </select>

        <p>"Selected value: " {value}</p>
    }
}