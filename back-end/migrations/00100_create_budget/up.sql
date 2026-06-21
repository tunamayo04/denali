CREATE TABLE budget_items (
    id SERIAL PRIMARY KEY,
    month DATE NOT NULL,
    category VARCHAR NOT NULL,
    budget_amount NUMERIC(12, 2) NOT NULL,
    actual_amount NUMERIC(12, 2) NOT NULL,
    color VARCHAR NOT NULL
);

INSERT INTO budget_items (month, category, budget_amount, actual_amount, color) VALUES
('2026-06-01', 'Groceries', 500.00, 450.00, '#FF5733'),
('2026-06-01', 'Rent', 1500.00, 1500.00, '#33FF57'),
('2026-06-01', 'Utilities', 200.00, 180.00, '#3357FF'),
('2026-06-01', 'Entertainment', 200.00, 250.00, '#FF33A1'),
('2026-06-01', 'Transportation', 150.00, 120.00, '#33FFF5');