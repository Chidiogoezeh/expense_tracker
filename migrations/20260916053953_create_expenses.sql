CREATE TABLE expenses (
    id UUID PRIMARY KEY,
    description TEXT NOT NULL,
    amount DOUBLE PRECISION NOT NULL,
    category TEXT NOT NULL
);