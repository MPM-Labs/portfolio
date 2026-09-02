use leptos::prelude::*;

#[cfg(target_arch = "wasm32")]
use gloo_net::http::Request;

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
#[derive(Clone, Debug)]
enum RoleLoadState {
    Loading,
    Loaded(String),
    Redirected,
    Error(String),
}

#[component]
pub fn SeeRole() -> impl IntoView {
    let state = RwSignal::new(RoleLoadState::Loading);

    #[cfg(target_arch = "wasm32")]
    {
        let state = state;
        leptos::task::spawn_local(async move {
            let next_state = match Request::get("/admin/get-role").send().await {
                Ok(response) if response.redirected() || response.url().ends_with("/login") => {
                    RoleLoadState::Redirected
                }
                Ok(response) if response.ok() => match response.text().await {
                    Ok(role) => RoleLoadState::Loaded(role),
                    Err(error) => RoleLoadState::Error(format!("Failed to read role: {error}")),
                },
                Ok(response) => RoleLoadState::Error(format!(
                    "Backend returned {} {}",
                    response.status(),
                    response.status_text()
                )),
                Err(error) => RoleLoadState::Error(format!("Request failed: {error}")),
            };

            state.set(next_state);
        });
    }

    view! {
        <section class="see-role">
            <h1>"See Role"</h1>
            {move || match state.get() {
                RoleLoadState::Loading => view! { <p>"Loading your role..."</p> }.into_any(),
                RoleLoadState::Loaded(role) => {
                    view! { <p>"Your role is " <strong>{role}</strong></p> }.into_any()
                }
                RoleLoadState::Redirected => {
                    view! {
                        <p>
                            "You were redirected to login, which means the backend did not see a valid session cookie."
                        </p>
                        <a href="/login">"Go to login"</a>
                    }
                        .into_any()
                }
                RoleLoadState::Error(message) => {
                    view! {
                        <p>"Failed to load role."</p>
                        <pre>{message}</pre>
                        <a href="/login">"Go to login"</a>
                    }
                        .into_any()
                }
            }}
        </section>
    }
}
