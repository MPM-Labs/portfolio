use leptos::prelude::*;
#[allow(unused)]
use leptos_router::{
    MatchNestedRoutes,
    any_nested_route::IntoAnyNestedRoute,
    components::{Outlet, Route},
    path,
};
use overview::Overview;

use see_role::SeeRole;

mod overview;
mod see_role;

#[component(transparent)]
pub fn AdminRoutes() -> impl MatchNestedRoutes + Clone {
    view! {
        <Route path=path!("") view=Overview />
        <Route path=path!("/overview") view=Overview />
        <Route path=path!("/see-role") view=SeeRole />
    }
    .into_inner()
    .into_any_nested_route()
}

#[component()]
pub fn Admin() -> impl IntoView {
    view! { <Outlet /> }
}
