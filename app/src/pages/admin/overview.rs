use leptos::prelude::*;
use leptos_router::components::A;

#[component()]
pub fn Overview() -> impl IntoView {
    view! {
        <section class="admin-overview">
            <h1>"Admin"</h1>
            <p>"Manage the portfolio projects and check the authenticated role."</p>
            <nav>
                <A href="projects">"Projects"</A>
                <A href="see-role">"See role"</A>
            </nav>
        </section>
    }
}
