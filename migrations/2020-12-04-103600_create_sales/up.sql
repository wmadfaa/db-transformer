-- Your SQL goes here
CREATE TABLE Sales (
    id TEXT PRIMARY KEY,
    product_id INTEGER NOT NULL REFERENCES Products,
    sale_date BIGINT NOT NULL,
    quantity DOUBLE PRECISION NOT NULL,
    unit TEXT NOT NULL
)