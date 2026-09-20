use crate::pages::{
    admin::AdminRoutes,
    login::Login,
    public::{Public, PublicRoutes},
};
use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, provide_meta_context};
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    path,
};
use pages::admin::Admin;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

pub mod models;
mod pages;
mod r#static;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/portfolio.css" />
        <Router>
            <Routes fallback=|| "Not found.">
                <ParentRoute path=path!("") view=Public>
                    <PublicRoutes />
                </ParentRoute>
                <ParentRoute path=path!("/admin") view=Admin>
                    <AdminRoutes />
                </ParentRoute>
                <Route path=path!("/login") view=Login />
            </Routes>
        </Router>
    }
}
