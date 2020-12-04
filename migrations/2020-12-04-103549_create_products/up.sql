-- Your SQL goes here
CREATE TABLE Products (
    id INTEGER PRIMARY KEY,
    category TEXT NOT NULL,
    name TEXT NOT NULL UNIQUE
)