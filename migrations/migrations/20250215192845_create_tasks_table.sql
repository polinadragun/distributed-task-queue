-- migrate:up
CREATE TABLE tasks (
                       id UUID PRIMARY KEY,
                       status TEXT NOT NULL,
                       data TEXT NOT NULL
);

