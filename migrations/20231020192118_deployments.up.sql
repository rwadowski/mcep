-- Add up migration script here

CREATE TABLE deployments (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    version VARCHAR NOT NULL,
    connections JSONB NOT NULL,
    sources JSONB NOT NULL,
    sinks JSONB NOT NULL,
    blocks JSONB NOT NULL
);