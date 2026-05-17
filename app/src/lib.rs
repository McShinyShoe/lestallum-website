#![recursion_limit = "512"]

mod components;
pub mod data;
mod layouts;
mod pages;

use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    ParamSegment, StaticSegment,
};

use crate::pages::area_detail::AreaDetailPage;
use crate::pages::areas::AreasPage;
use crate::pages::home::HomePage;
use crate::pages::not_found::NotFoundPage;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en" data-theme="dark">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <link rel="manifest" href="/site.webmanifest"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body class="bg-base-100 text-base-content antialiased">
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/lestallum-website.css"/>
        <Title text="Lestallum Town"/>

        <Router>
            <main>
                <Routes fallback=|| view! { <NotFoundPage/> }>
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=StaticSegment("areas") view=AreasPage/>
                    <Route path=(StaticSegment("areas"), ParamSegment("area")) view=AreaDetailPage/>
                </Routes>
            </main>
        </Router>
    }
}
