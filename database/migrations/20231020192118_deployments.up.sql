-- Add up migration script here

CREATE TABLE deployments (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    version VARCHAR NOT NULL,
    application_id INT NOT NULL
);

ALTER TABLE deployments ADD CONSTRAINT fk_application_id FOREIGN KEY (application_id) REFERENCES app_definitions (id);