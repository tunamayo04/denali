-- Your SQL goes here
CREATE TYPE account_type AS ENUM ('Checking', 'Saving', 'Credit Card', 'Investment', 'Cash', 'Loan');

CREATE TABLE accounts (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    institution_name VARCHAR NOT NULL,
    account_type account_type NOT NULL,
    currency CHAR(3) NOT NULL,
    opening_balance NUMERIC(12, 2) NOT NULL,
    current_balance NUMERIC(12, 2) NOT NULL,
    is_closed BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

INSERT INTO accounts (name, institution_name, account_type, currency, opening_balance, current_balance) VALUES
('Main Checking', 'Chase', 'Checking', 'USD', 1000.00, 1200.00),
('Savings', 'Ally', 'Saving', 'USD', 5000.00, 5050.00),
('Travel Card', 'Amex', 'Credit Card', 'USD', 0.00, -250.00),
('Cash Wallet', 'None', 'Cash', 'USD', 50.00, 45.00),
('Auto Loan', 'Wells Fargo', 'Loan', 'USD', -25000.00, -22000.00);