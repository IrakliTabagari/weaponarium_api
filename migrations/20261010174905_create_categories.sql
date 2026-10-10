-- Add migration script here
CREATE TABLE categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR(250) NOT NULL,
    parent_id INT REFERENCES categories(id),
    CONSTRAINT categories_not_own_parent
        CHECK (parent_id <> id)
);

CREATE INDEX ix_categories_parent_id ON categories(parent_id);
