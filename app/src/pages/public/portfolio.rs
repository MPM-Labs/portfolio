use leptos::prelude::*;

use crate::r#static::project::ProjectCard;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    Planned,
    InProgress,
    Completed,
    Archived,
}

impl Status {
    pub fn label(&self) -> &'static str {
        match self {
            Status::Planned => "Planned",
            Status::InProgress => "In Progress",
            Status::Completed => "Completed",
            Status::Archived => "Archived",
        }
    }

    pub fn css_class(&self) -> &'static str {
        match self {
            Status::Planned => "status-planned",
            Status::InProgress => "status-in-progress",
            Status::Completed => "status-completed",
            Status::Archived => "status-archived",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Project {
    pub title: String,
    pub languages: Vec<String>,
    pub description_line: String,
    pub status: Status,
    pub collaborative: bool,
    pub repo_url: Option<String>,
    pub live_url: Option<String>,
}

pub fn sample_projects() -> Vec<Project> {
    vec![
        Project {
            title: "Rust Task Scheduler".to_string(),
            languages: vec!["Rust".to_string(), "Tokio".to_string()],
            description_line: "An async job scheduler with retry logic and priority queues."
                .to_string(),
            status: Status::Completed,
            collaborative: false,
            repo_url: Some("https://github.com/you/task-scheduler".to_string()),
            live_url: None,
        },
        Project {
            title: "Leptos Portfolio Site".to_string(),
            languages: vec!["Rust".to_string(), "Leptos".to_string(), "CSS".to_string()],
            description_line: "This very site — server-rendered with islands of interactivity."
                .to_string(),
            status: Status::InProgress,
            collaborative: false,
            repo_url: Some("https://github.com/you/portfolio".to_string()),
            live_url: Some("https://yoursite.dev".to_string()),
        },
        Project {
            title: "Distributed KV Store".to_string(),
            languages: vec!["Go".to_string(), "gRPC".to_string()],
            description_line: "A Raft-based key-value store built with two classmates.".to_string(),
            status: Status::Completed,
            collaborative: true,
            repo_url: Some("https://github.com/you/kv-store".to_string()),
            live_url: None,
        },
        Project {
            title: "ML Model Playground".to_string(),
            languages: vec!["Python".to_string(), "PyTorch".to_string()],
            description_line: "Experiments in fine-tuning small vision transformers.".to_string(),
            status: Status::Archived,
            collaborative: false,
            repo_url: None,
            live_url: None,
        },
        Project {
            title: "Realtime Chat App".to_string(),
            languages: vec![
                "TypeScript".to_string(),
                "WebSockets".to_string(),
                "React".to_string(),
            ],
            description_line: "A chat app with presence indicators and message history."
                .to_string(),
            status: Status::Planned,
            collaborative: true,
            repo_url: None,
            live_url: None,
        },
        Project {
            title: "Rust Task Scheduler".to_string(),
            languages: vec!["Rust".to_string(), "Tokio".to_string()],
            description_line: "An async job scheduler with retry logic and priority queues."
                .to_string(),
            status: Status::Completed,
            collaborative: false,
            repo_url: Some("https://github.com/you/task-scheduler".to_string()),
            live_url: None,
        },
        Project {
            title: "Leptos Portfolio Site".to_string(),
            languages: vec!["Rust".to_string(), "Leptos".to_string(), "CSS".to_string()],
            description_line: "This very site — server-rendered with islands of interactivity."
                .to_string(),
            status: Status::InProgress,
            collaborative: false,
            repo_url: Some("https://github.com/you/portfolio".to_string()),
            live_url: Some("https://yoursite.dev".to_string()),
        },
        Project {
            title: "Distributed KV Store".to_string(),
            languages: vec!["Go".to_string(), "gRPC".to_string()],
            description_line: "A Raft-based key-value store built with two classmates.".to_string(),
            status: Status::Completed,
            collaborative: true,
            repo_url: Some("https://github.com/you/kv-store".to_string()),
            live_url: None,
        },
        Project {
            title: "ML Model Playground".to_string(),
            languages: vec!["Python".to_string(), "PyTorch".to_string()],
            description_line: "Experiments in fine-tuning small vision transformers.".to_string(),
            status: Status::Archived,
            collaborative: false,
            repo_url: None,
            live_url: None,
        },
        Project {
            title: "Realtime Chat App".to_string(),
            languages: vec![
                "TypeScript".to_string(),
                "WebSockets".to_string(),
                "React".to_string(),
            ],
            description_line: "A chat app with presence indicators and message history."
                .to_string(),
            status: Status::Planned,
            collaborative: true,
            repo_url: None,
            live_url: None,
        },
        Project {
            title: "Rust Task Scheduler".to_string(),
            languages: vec!["Rust".to_string(), "Tokio".to_string()],
            description_line: "An async job scheduler with retry logic and priority queues."
                .to_string(),
            status: Status::Completed,
            collaborative: false,
            repo_url: Some("https://github.com/you/task-scheduler".to_string()),
            live_url: None,
        },
        Project {
            title: "Leptos Portfolio Site".to_string(),
            languages: vec!["Rust".to_string(), "Leptos".to_string(), "CSS".to_string()],
            description_line: "This very site — server-rendered with islands of interactivity."
                .to_string(),
            status: Status::InProgress,
            collaborative: false,
            repo_url: Some("https://github.com/you/portfolio".to_string()),
            live_url: Some("https://yoursite.dev".to_string()),
        },
        Project {
            title: "Distributed KV Store".to_string(),
            languages: vec!["Go".to_string(), "gRPC".to_string()],
            description_line: "A Raft-based key-value store built with two classmates.".to_string(),
            status: Status::Completed,
            collaborative: true,
            repo_url: Some("https://github.com/you/kv-store".to_string()),
            live_url: None,
        },
        Project {
            title: "ML Model Playground".to_string(),
            languages: vec!["Python".to_string(), "PyTorch".to_string()],
            description_line: "Experiments in fine-tuning small vision transformers.".to_string(),
            status: Status::Archived,
            collaborative: false,
            repo_url: None,
            live_url: None,
        },
        Project {
            title: "Realtime Chat App".to_string(),
            languages: vec![
                "TypeScript".to_string(),
                "WebSockets".to_string(),
                "React".to_string(),
            ],
            description_line: "A chat app with presence indicators and message history."
                .to_string(),
            status: Status::Planned,
            collaborative: true,
            repo_url: None,
            live_url: None,
        },
        Project {
            title: "Rust Task Scheduler".to_string(),
            languages: vec!["Rust".to_string(), "Tokio".to_string()],
            description_line: "An async job scheduler with retry logic and priority queues."
                .to_string(),
            status: Status::Completed,
            collaborative: false,
            repo_url: Some("https://github.com/you/task-scheduler".to_string()),
            live_url: None,
        },
        Project {
            title: "Leptos Portfolio Site".to_string(),
            languages: vec!["Rust".to_string(), "Leptos".to_string(), "CSS".to_string()],
            description_line: "This very site — server-rendered with islands of interactivity."
                .to_string(),
            status: Status::InProgress,
            collaborative: false,
            repo_url: Some("https://github.com/you/portfolio".to_string()),
            live_url: Some("https://yoursite.dev".to_string()),
        },
        Project {
            title: "Distributed KV Store".to_string(),
            languages: vec!["Go".to_string(), "gRPC".to_string()],
            description_line: "A Raft-based key-value store built with two classmates.".to_string(),
            status: Status::Completed,
            collaborative: true,
            repo_url: Some("https://github.com/you/kv-store".to_string()),
            live_url: None,
        },
        Project {
            title: "ML Model Playground".to_string(),
            languages: vec!["Python".to_string(), "PyTorch".to_string()],
            description_line: "Experiments in fine-tuning small vision transformers.".to_string(),
            status: Status::Archived,
            collaborative: false,
            repo_url: None,
            live_url: None,
        },
        Project {
            title: "Realtime Chat App".to_string(),
            languages: vec![
                "TypeScript".to_string(),
                "WebSockets".to_string(),
                "React".to_string(),
            ],
            description_line: "A chat app with presence indicators and message history."
                .to_string(),
            status: Status::Planned,
            collaborative: true,
            repo_url: None,
            live_url: None,
        },
    ]
}

#[component]
pub fn Portfolio() -> impl IntoView {
    let projects = sample_projects();

    view! {
        <section class="full portfolio-intro">
            <h1>"Portfolio Projects"</h1>
            <p>"I'm always working on it, but you can see some of them here."</p>
        </section>
        <section class="full projects-grid portfolio-projects">
            <For
                each=move || projects.clone()
                key=|p| p.title.clone()
                children=move |project: Project| {
                    view! { <ProjectCard project=project /> }
                }
            />
        </section>
    }
}
