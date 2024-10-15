-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE users (
    id UUID PRIMARY KEY default uuid_generate_v4(),
    name TEXT NOT NULL,
    email TEXT NOT NULL
);