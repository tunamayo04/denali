-- Your SQL goes here
CREATE TABLE transactions (
    id SERIAL PRIMARY KEY,
    vendor_id SERIAL REFERENCES vendors(id) NOT NULL,
    amount NUMERIC(12, 2) NOT NULL,
    date DATE NOT NULL
)