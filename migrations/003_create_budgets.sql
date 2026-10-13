CREATE TABLE budgets (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    category TEXT NOT NULL
        CHECK (char_length(btrim(category)) BETWEEN 1 AND 50),

    amount_limit NUMERIC(19, 4) NOT NULL
        CHECK (amount_limit > 0),

    currency CHAR(3) NOT NULL DEFAULT 'NGN'
        CHECK (currency ~ '^[A-Z]{3}$'),

    month DATE NOT NULL
        CHECK (EXTRACT(DAY FROM month) = 1),

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE (user_id, category, currency, month)
);