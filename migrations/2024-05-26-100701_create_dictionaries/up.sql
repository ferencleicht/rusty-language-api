-- Your SQL goes here
CREATE TABLE dictionaries (
    id SERIAL PRIMARY KEY,
    language TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE translations (
    id SERIAL PRIMARY KEY,
    dictionary_id INTEGER NOT NULL REFERENCES dictionaries(id) ON DELETE CASCADE,
    source TEXT NOT NULL,
    target TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
)
