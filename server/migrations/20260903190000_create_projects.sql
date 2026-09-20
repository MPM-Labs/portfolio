CREATE TABLE projects (
    id BIGSERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    languages TEXT NOT NULL,
    description_line TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('planned', 'in_progress', 'completed', 'archived')),
    collaborative BOOLEAN NOT NULL DEFAULT FALSE,
    repo_url TEXT,
    live_url TEXT
);

INSERT INTO projects (title, languages, description_line, status, collaborative, repo_url, live_url)
VALUES
    (
        'Rust Task Scheduler',
        'Rust,Tokio',
        'An async job scheduler with retry logic and priority queues.',
        'completed',
        FALSE,
        'https://github.com/you/task-scheduler',
        NULL
    ),
    (
        'Leptos Portfolio Site',
        'Rust,Leptos,CSS',
        'This very site - server-rendered with islands of interactivity.',
        'in_progress',
        FALSE,
        'https://github.com/you/portfolio',
        'https://yoursite.dev'
    ),
    (
        'Distributed KV Store',
        'Go,gRPC',
        'A Raft-based key-value store built with two classmates.',
        'completed',
        TRUE,
        'https://github.com/you/kv-store',
        NULL
    ),
    (
        'ML Model Playground',
        'Python,PyTorch',
        'Experiments in fine-tuning small vision transformers.',
        'archived',
        FALSE,
        NULL,
        NULL
    ),
    (
        'Realtime Chat App',
        'TypeScript,WebSockets,React',
        'A chat app with presence indicators and message history.',
        'planned',
        TRUE,
        NULL,
        NULL
    );