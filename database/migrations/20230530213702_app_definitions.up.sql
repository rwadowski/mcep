-- Add up migration script here

CREATE TABLE definitions (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    version VARCHAR NOT NULL,
    body TEXT NOT NULL,
    description TEXT,
    help TEXT
);
