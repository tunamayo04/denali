CREATE TABLE budget_items (
    id SERIAL PRIMARY KEY,
    month DATE NOT NULL,
    category VARCHAR NOT NULL,
    budget_amount NUMERIC(12, 2) NOT NULL,
    actual_amount NUMERIC(12, 2) NOT NULL,
    color VARCHAR NOT NULL
)