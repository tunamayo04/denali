-- Your SQL goes here
CREATE TABLE transactions (
    id SERIAL PRIMARY KEY,
    vendor_id SERIAL REFERENCES vendors(id) NOT NULL,
    account_id SERIAL REFERENCES accounts(id) NOT NULL,
    amount NUMERIC(12, 2) NOT NULL,
    date DATE NOT NULL
);

INSERT INTO transactions (vendor_id, account_id, amount, date) VALUES
(1, 1, -45.99, '2026-06-15'),
(2, 2, -120.50, '2026-06-15'),
(3, 3, -5.75, '2026-06-17'),
(4, 3, 60.00, '2026-06-18'),
(5, 5, -15.99, '2026-06-19'),
(6, 4, -85.20, '2026-06-20'),
(7, 1, -999.00, '2026-06-20');