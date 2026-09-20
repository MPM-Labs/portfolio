use crate::pages::public::portfolio::Project;
use leptos::prelude::*;

#[component]
pub fn ProjectCard(project: Project) -> impl IntoView {
    view! {
        <div class="project-card">
            <div class="title">
                <h2>{project.title.clone()}</h2>
                <p class=format!(
                    "status-badge {}",
                    project.status.css_class(),
                )>"Status: "{project.status.label()}</p>
            </div>
            <ul class="languages">
                <For
                    each=move || project.languages.clone()
                    key=|lang| lang.clone()
                    children=move |lang: String| view! { <li>{lang}</li> }
                />
            </ul>
            <p class="description">{project.description_line.clone()}</p>
            <div class="links">
                {project
                    .repo_url
                    .clone()
                    .map(|url| {
                        view! {
                            <a href=url target="_blank">
                                "Repository"
                            </a>
                        }
                    })}
                {project
                    .live_url
                    .clone()
                    .map(|url| {
                        view! {
                            <a href=url target="_blank">
                                "Live Artifact"
                            </a>
                        }
                    })}
            </div>
        </div>
    }
}

// {project.collaborative.then(|| view! { <p class="collab-tag">"Collaborative"</p> })}
