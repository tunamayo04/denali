-- Your SQL goes here
CREATE TABLE vendors
(
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL
);

INSERT INTO vendors (name) VALUES
('Amazon'),
('Walmart'),
('Starbucks'),
('Desjardins'),
('Netflix'),
('IGA'),
('Apple');