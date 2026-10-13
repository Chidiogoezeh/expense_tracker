CREATE TABLE transactions (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    type TEXT NOT NULL
        CHECK (type IN ('INCOME', 'EXPENSE')),

    description TEXT NOT NULL
        CHECK (char_length(btrim(description)) BETWEEN 1 AND 200),

    amount NUMERIC(19, 4) NOT NULL
        CHECK (amount > 0),

    currency CHAR(3) NOT NULL DEFAULT 'NGN'
        CHECK (currency ~ '^[A-Z]{3}$'),

    category TEXT NOT NULL
        CHECK (char_length(btrim(category)) BETWEEN 1 AND 50),

    occurred_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);