CREATE INDEX idx_transactions_user_occurred
    ON transactions (user_id, occurred_at DESC, id DESC);

CREATE INDEX idx_transactions_user_type_occurred
    ON transactions (user_id, type, occurred_at DESC, id DESC);

CREATE INDEX idx_transactions_user_category
    ON transactions (user_id, category);

CREATE INDEX idx_budgets_user_month
    ON budgets (user_id, month);