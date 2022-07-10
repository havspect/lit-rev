-- Add migration script here
-- @block create bibliography table
-- @conn SQLITE
CREATE TABLE IF NOT EXISTS bibliography (
    ID INTEGER PRIMARY KEY NOT NULL,
    ENTRY_TYPE TEXT NOT NULL,
    AUTHOR TEXT,
    TITLE TEXT NOT NULL UNIQUE ON CONFLICT FAIL
);


