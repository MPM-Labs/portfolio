use crate::r#static::project::ProjectCard;
use leptos::prelude::*;
use leptos::server;
use leptos::server_fn::ServerFnError;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use axum::Extension;
#[cfg(feature = "ssr")]
use leptos_axum::extract;
#[cfg(feature = "ssr")]
use sqlx::{Pool, Postgres};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ProjectStatus {
    #[default]
    Planned,
    InProgress,
    Completed,
    Archived,
}

impl ProjectStatus {
    pub const ALL: [Self; 4] = [
        Self::Planned,
        Self::InProgress,
        Self::Completed,
        Self::Archived,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::Planned => "Planned",
            Self::InProgress => "In Progress",
            Self::Completed => "Completed",
            Self::Archived => "Archived",
        }
    }

    pub fn css_class(self) -> &'static str {
        match self {
            Self::Planned => "status-planned",
            Self::InProgress => "status-in-progress",
            Self::Completed => "status-completed",
            Self::Archived => "status-archived",
        }
    }

    pub fn as_db_value(self) -> &'static str {
        match self {
            Self::Planned => "planned",
            Self::InProgress => "in_progress",
            Self::Completed => "completed",
            Self::Archived => "archived",
        }
    }

    pub fn from_db_value(value: &str) -> Option<Self> {
        match value {
            "planned" => Some(Self::Planned),
            "in_progress" => Some(Self::InProgress),
            "completed" => Some(Self::Completed),
            "archived" => Some(Self::Archived),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Project {
    pub id: i64,
    pub title: String,
    pub languages: Vec<String>,
    pub description_line: String,
    pub status: ProjectStatus,
    pub collaborative: bool,
    pub repo_url: Option<String>,
    pub live_url: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDraft {
    pub id: Option<i64>,
    pub title: String,
    pub languages: String,
    pub description_line: String,
    pub status: String,
    pub collaborative: bool,
    pub repo_url: String,
    pub live_url: String,
}

impl ProjectDraft {
    pub fn from_project(project: Project) -> Self {
        Self {
            id: Some(project.id),
            title: project.title,
            languages: project.languages.join(", "),
            description_line: project.description_line,
            status: project.status.as_db_value().to_string(),
            collaborative: project.collaborative,
            repo_url: project.repo_url.unwrap_or_default(),
            live_url: project.live_url.unwrap_or_default(),
        }
    }
}

#[cfg(feature = "ssr")]
#[derive(sqlx::FromRow)]
struct ProjectRow {
    id: i64,
    title: String,
    languages: String,
    description_line: String,
    status: String,
    collaborative: bool,
    repo_url: Option<String>,
    live_url: Option<String>,
}

#[cfg(feature = "ssr")]
impl From<ProjectRow> for Project {
    fn from(row: ProjectRow) -> Self {
        Self {
            id: row.id,
            title: row.title,
            languages: split_languages(&row.languages),
            description_line: row.description_line,
            status: ProjectStatus::from_db_value(&row.status).unwrap_or_default(),
            collaborative: row.collaborative,
            repo_url: row.repo_url,
            live_url: row.live_url,
        }
    }
}

fn split_languages(languages: &str) -> Vec<String> {
    languages
        .split(',')
        .map(str::trim)
        .filter(|language| !language.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

fn join_languages(languages: &str) -> Vec<String> {
    split_languages(languages)
}

fn normalize_optional_url(value: String) -> Option<String> {
    let trimmed = value.trim();
    (!trimmed.is_empty()).then(|| trimmed.to_string())
}

#[server]
pub async fn list_projects() -> Result<Vec<Project>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let Extension(pool): Extension<Pool<Postgres>> = extract().await?;
        let rows = sqlx::query_as::<_, ProjectRow>(
            r#"
            SELECT id, title, languages, description_line, status, collaborative, repo_url, live_url
            FROM projects
            ORDER BY id DESC
            "#,
        )
        .fetch_all(&pool)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

        Ok(rows.into_iter().map(Project::from).collect())
    }

    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new(
            "list_projects can only run on the server",
        ))
    }
}

#[server(prefix = "/admin")]
pub async fn save_project(project: ProjectDraft) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let Extension(pool): Extension<Pool<Postgres>> = extract().await?;
        let title = project.title.trim();
        let description_line = project.description_line.trim();
        let status = ProjectStatus::from_db_value(&project.status)
            .ok_or_else(|| ServerFnError::new("project status is invalid"))?;
        let languages = join_languages(&project.languages);
        let repo_url = normalize_optional_url(project.repo_url);
        let live_url = normalize_optional_url(project.live_url);

        if title.is_empty() {
            return Err(ServerFnError::new("project title is required"));
        }

        if description_line.is_empty() {
            return Err(ServerFnError::new("project description is required"));
        }

        if languages.is_empty() {
            return Err(ServerFnError::new("at least one language is required"));
        }

        match project.id {
            Some(id) => {
                sqlx::query(
                    r#"
                    UPDATE projects
                    SET title = $1,
                        languages = $2,
                        description_line = $3,
                        status = $4,
                        collaborative = $5,
                        repo_url = $6,
                        live_url = $7
                    WHERE id = $8
                    "#,
                )
                .bind(title)
                .bind(languages.join(", "))
                .bind(description_line)
                .bind(status.as_db_value())
                .bind(project.collaborative)
                .bind(repo_url)
                .bind(live_url)
                .bind(id)
                .execute(&pool)
                .await
                .map_err(|error| ServerFnError::new(error.to_string()))?;
            }
            None => {
                sqlx::query(
                    r#"
                    INSERT INTO projects (
                        title,
                        languages,
                        description_line,
                        status,
                        collaborative,
                        repo_url,
                        live_url
                    )
                    VALUES ($1, $2, $3, $4, $5, $6, $7)
                    "#,
                )
                .bind(title)
                .bind(languages.join(", "))
                .bind(description_line)
                .bind(status.as_db_value())
                .bind(project.collaborative)
                .bind(repo_url)
                .bind(live_url)
                .execute(&pool)
                .await
                .map_err(|error| ServerFnError::new(error.to_string()))?;
            }
        }

        Ok(())
    }

    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new(
            "save_project can only run on the server",
        ))
    }
}

#[server(prefix = "/admin")]
pub async fn delete_project(id: i64) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let Extension(pool): Extension<Pool<Postgres>> = extract().await?;
        sqlx::query("DELETE FROM projects WHERE id = $1")
            .bind(id)
            .execute(&pool)
            .await
            .map_err(|error| ServerFnError::new(error.to_string()))?;

        Ok(())
    }

    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new(
            "delete_project can only run on the server",
        ))
    }
}

#[component]
pub fn Portfolio() -> impl IntoView {
    let refresh = RwSignal::new(0usize);
    let projects = Resource::new(move || refresh.get(), |_| list_projects());

    view! {
        <section class="full portfolio-intro">
            <h1>"Portfolio Projects"</h1>
            <p>"I'm always working on it, but you can see some of them here."</p>
        </section>
        <section class="full projects-grid portfolio-projects">
            <Suspense fallback=move || {
                view! { <p>"Loading projects..."</p> }
            }>
                {move || match projects.get() {
                    Some(Ok(projects)) if projects.is_empty() => {
                        view! { <p>"No projects yet."</p> }.into_any()
                    }
                    Some(Ok(projects)) => {
                        view! {
                            <For
                                each=move || projects.clone()
                                key=|project| project.id
                                children=move |project: Project| {
                                    view! { <ProjectCard project=project /> }
                                }
                            />
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
                    None => view! { <p>"Loading projects..."</p> }.into_any(),
                }}
            </Suspense>
        </section>
    }
}
