CREATE TYPE role AS ENUM ('superuser', 'admin', 'contributor', 'user');

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    name TEXT,
    email TEXT NOT NULL,
    role role NOT NULL
);
