CREATE TABLE users (
    id UUID PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

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

CREATE INDEX idx_transactions_user_occurred
    ON transactions (user_id, occurred_at DESC, id DESC);

CREATE INDEX idx_transactions_user_type_occurred
    ON transactions (user_id, type, occurred_at DESC, id DESC);

CREATE INDEX idx_transactions_user_category
    ON transactions (user_id, category);

CREATE INDEX idx_budgets_user_month
    ON budgets (user_id, month);