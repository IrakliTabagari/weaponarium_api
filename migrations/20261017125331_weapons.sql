CREATE TABLE weapons (
    id SERIAL PRIMARY KEY,
    name VARCHAR(250) NOT NULL,
    category_id INT REFERENCES categories(id)
);

CREATE INDEX ix_weapons_category_id ON weapons(category_id);
