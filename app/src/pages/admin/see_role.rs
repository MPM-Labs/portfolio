use leptos::prelude::*;
use leptos::server;
use leptos::server_fn::ServerFnError;

#[cfg(feature = "ssr")]
use axum::Extension;

#[cfg(feature = "ssr")]
use leptos_axum::extract;

#[server(endpoint = "get-role")]
pub async fn get_role() -> Result<String, ServerFnError> {
    let Extension(role): Extension<String> = extract().await?;
    Ok(role)
}

#[component]
pub fn SeeRole() -> impl IntoView {
    let role = Resource::new(|| (), |_| get_role());

    view! {
        <section class="see-role">
            <h1>"See Role"</h1>
            <Suspense fallback=move || view! { <p>"Loading your role..."</p> }>
                {move || match role.get() {
                    Some(Ok(role)) => {
                        view! { <p>"Your role is " <strong>{role}</strong></p> }.into_any()
                    }
                    Some(Err(error)) => {
                        view! {
                            <p>"Failed to load role."</p>
                            <pre>{format!("{error:?}")}</pre>
                            <a href="/login">"Go to login"</a>
                        }
                            .into_any()
                    }
                    None => view! { <p>"Loading your role..."</p> }.into_any(),
                }}
            </Suspense>
        </section>
    }
}
