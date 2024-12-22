-- @block
-- Creates the db Schema
CREATE EXTENSION IF NOT EXISTS vector;
CREATE TABLE IF NOT EXISTS images (
    id UUID DEFAULT gen_random_uuid(),
    data BYTEA NOT NULL,
    description TEXT,
    embedding vector(768),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id)
);
-- @block
-- Delete the db
DROP TABLE IF EXISTS images;
-- @block
-- Tests