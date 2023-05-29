-- Add migration script here

CREATE TABLE app_definitions (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    version VARCHAR NOT NULL,
    body TEXT NOT NULL,
    description TEXT,
    help TEXT
);