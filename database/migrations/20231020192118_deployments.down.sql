-- Add down migration script here

ALTER TABLE deployments DROP CONSTRAINT fk_application_id;

DROP TABLE deployments;