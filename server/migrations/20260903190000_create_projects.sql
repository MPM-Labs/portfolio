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