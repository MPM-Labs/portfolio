use crate::pages::public::portfolio::{
    Project, ProjectDraft, ProjectStatus, delete_project, list_projects, save_project,
};
use crate::r#static::project::ProjectCard;
use leptos::prelude::*;

#[component]
pub fn Projects() -> impl IntoView {
    let refresh = RwSignal::new(0usize);
    let projects = Resource::new(move || refresh.get(), |_| list_projects());
    let draft = RwSignal::new(ProjectDraft::default());
    let message = RwSignal::new(String::new());

    let clear_draft = move |_| {
        draft.set(ProjectDraft::default());
        message.set(String::new());
    };

    let save_current = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let payload = draft.get_untracked();
        let draft = draft;
        let refresh = refresh;
        let message = message;

        leptos::task::spawn_local(async move {
            match save_project(payload).await {
                Ok(()) => {
                    message.set("Project saved.".to_string());
                    refresh.update(|value| *value += 1);
                    draft.set(ProjectDraft::default());
                }
                Err(error) => {
                    message.set(format!("Failed to save project: {error:?}"));
                }
            }
        });
    };

    let start_new = move |_| {
        draft.set(ProjectDraft::default());
        message.set("Creating a new project.".to_string());
    };

    view! {
        <section class="full projects-admin">
            <div class="projects-admin__intro">
                <h1>"Projects"</h1>
                <p>"Create, edit, and remove portfolio projects from the database."</p>
            </div>

            <Suspense fallback=move || {
                view! { <p>"Loading project editor..."</p> }
            }>
                {move || match projects.get() {
                    Some(Ok(projects)) => {
                        let current_draft = draft.get();
                        let draft_title = move || {
                            if current_draft.id.is_some() {
                                "Edit project"
                            } else {
                                "Create project"
                            }
                        };

                        view! {
                            <div class="projects-admin__layout">
                                <form class="project-editor" on:submit=save_current>
                                    <div class="project-editor__header">
                                        <h2>{draft_title}</h2>
                                        <button type="button" on:click=clear_draft>
                                            "Clear"
                                        </button>
                                    </div>

                                    <label>
                                        <span>"Title"</span>
                                        <input
                                            type="text"
                                            prop:value=move || draft.get().title.clone()
                                            on:input=move |ev| {
                                                draft.update(|draft| draft.title = event_target_value(&ev))
                                            }
                                        />
                                    </label>

                                    <label>
                                        <span>"Languages"</span>
                                        <input
                                            type="text"
                                            placeholder="Rust, Tokio, Axum"
                                            prop:value=move || draft.get().languages.clone()
                                            on:input=move |ev| {
                                                draft
                                                    .update(|draft| draft.languages = event_target_value(&ev))
                                            }
                                        />
                                    </label>

                                    <label>
                                        <span>"Description"</span>
                                        <textarea
                                            rows="4"
                                            prop:value=move || draft.get().description_line.clone()
                                            on:input=move |ev| {
                                                draft
                                                    .update(|draft| {
                                                        draft.description_line = event_target_value(&ev);
                                                    })
                                            }
                                        ></textarea>
                                    </label>

                                    <label>
                                        <span>"Status"</span>
                                        <select
                                            prop:value=move || draft.get().status.clone()
                                            on:change=move |ev| {
                                                draft.update(|draft| draft.status = event_target_value(&ev))
                                            }
                                        >
                                            <For
                                                each=move || {
                                                    ProjectStatus::ALL.into_iter().collect::<Vec<_>>()
                                                }
                                                key=|status| status.as_db_value()
                                                children=move |status: ProjectStatus| {
                                                    view! {
                                                        <option value=status.as_db_value()>{status.label()}</option>
                                                    }
                                                }
                                            />
                                        </select>
                                    </label>

                                    <label class="project-editor__checkbox">
                                        <input
                                            type="checkbox"
                                            prop:checked=move || draft.get().collaborative
                                            on:change=move |ev| {
                                                draft
                                                    .update(|draft| {
                                                        draft.collaborative = event_target_checked(&ev);
                                                    })
                                            }
                                        />
                                        <span>"Collaborative project"</span>
                                    </label>

                                    <label>
                                        <span>"Repository URL"</span>
                                        <input
                                            type="url"
                                            placeholder="https://github.com/..."
                                            prop:value=move || draft.get().repo_url.clone()
                                            on:input=move |ev| {
                                                draft
                                                    .update(|draft| draft.repo_url = event_target_value(&ev))
                                            }
                                        />
                                    </label>

                                    <label>
                                        <span>"Live URL"</span>
                                        <input
                                            type="url"
                                            placeholder="https://..."
                                            prop:value=move || draft.get().live_url.clone()
                                            on:input=move |ev| {
                                                draft
                                                    .update(|draft| draft.live_url = event_target_value(&ev))
                                            }
                                        />
                                    </label>

                                    <input
                                        type="hidden"
                                        prop:value=move || {
                                            draft.get().id.map(|id| id.to_string()).unwrap_or_default()
                                        }
                                    />

                                    <div class="project-editor__actions">
                                        <button type="submit">
                                            {move || {
                                                if draft.get().id.is_some() {
                                                    "Update project"
                                                } else {
                                                    "Create project"
                                                }
                                            }}
                                        </button>
                                        <button type="button" on:click=start_new>
                                            "New blank project"
                                        </button>
                                    </div>
                                </form>

                                <div class="projects-admin__message">
                                    {move || {
                                        if message.get().is_empty() {
                                            view! { <p>""</p> }.into_any()
                                        } else {
                                            view! { <p>{message.get()}</p> }.into_any()
                                        }
                                    }}
                                </div>

                                <div class="projects-admin__list">
                                    <h2>"Current projects"</h2>
                                    <For
                                        each=move || projects.clone()
                                        key=|project| project.id
                                        children=move |project: Project| {
                                            let edit_project = project.clone();
                                            let delete_id = project.id;

                                            view! {
                                                <article class="project-admin-card">
                                                    <ProjectCard project=project />
                                                    <div class="project-admin-card__actions">
                                                        <button
                                                            type="button"
                                                            on:click=move |_| {
                                                                draft.set(ProjectDraft::from_project(edit_project.clone()));
                                                                message.set(format!("Editing {}.", edit_project.title));
                                                            }
                                                        >
                                                            "Edit"
                                                        </button>
                                                        <button
                                                            type="button"
                                                            on:click=move |_| {
                                                                let refresh = refresh;
                                                                let draft = draft;
                                                                let message = message;
                                                                let selected_id = draft.get_untracked().id;
                                                                leptos::task::spawn_local(async move {
                                                                    match delete_project(delete_id).await {
                                                                        Ok(()) => {
                                                                            if selected_id == Some(delete_id) {
                                                                                draft.set(ProjectDraft::default());
                                                                            }
                                                                            refresh.update(|value| *value += 1);
                                                                            message.set("Project deleted.".to_string());
                                                                        }
                                                                        Err(error) => {
                                                                            message.set(format!("Failed to delete project: {error:?}"));
                                                                        }
                                                                    }
                                                                });
                                                            }
                                                        >
                                                            "Delete"
                                                        </button>
                                                    </div>
                                                </article>
                                            }
                                        }
                                    />
                                </div>
                            </div>
                        }
                            .into_any()
                    }
                    Some(Err(error)) => {
                        view! {
                            <p>"Failed to load projects."</p>
                            <pre>{format!("{error:?}")}</pre>
                        }
                            .into_any()
                    }
                    None => view! { <p>"Loading project editor..."</p> }.into_any(),
                }}
            </Suspense>
        </section>
    }
}
