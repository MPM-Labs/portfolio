use crate::pages::public::portfolio::Project;
use leptos::prelude::*;

#[component]
pub fn ProjectCard(project: Project) -> impl IntoView {
    view! {
        <div>
            <h2>{project.title.clone()}</h2>
            <span class=format!(
                "status-badge {}",
                project.status.css_class(),
            )>{project.status.label()}</span>
            <p class="description">{project.description_line.clone()}</p>
            <ul class="languages">
                <For
                    each=move || project.languages.clone()
                    key=|lang| lang.clone()
                    children=move |lang: String| view! { <li>{lang}</li> }
                />
            </ul>
            {project.collaborative.then(|| view! { <p class="collab-tag">"Collaborative"</p> })}
            {project
                .repo_url
                .clone()
                .map(|url| {
                    view! {
                        <a href=url target="_blank">
                            "Repo"
                        </a>
                    }
                })}
            {project
                .live_url
                .clone()
                .map(|url| {
                    view! {
                        <a href=url target="_blank">
                            "Live"
                        </a>
                    }
                })}
        </div>
    }
}
