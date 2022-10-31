DROP DATABASE IF EXISTS nodeforum;
CREATE DATABASE nodeforum;

DROP SCHEMA IF EXISTS nodeforum CASCADE;
CREATE SCHEMA nodeforum;

CREATE TABLE nodeforum.users (
    id BIGSERIAL PRIMARY KEY,
    email TEXT NOT NULL,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    user_password TEXT NOT NULL,
    username TEXT UNIQUE NOT NULL,
    UNIQUE (username)
);

CREATE TABLE nodeforum.user_settings (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGSERIAL,
    CONSTRAINT user_id FOREIGN KEY (user_id)
        REFERENCES nodeforum.users (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
        NOT VALID
);