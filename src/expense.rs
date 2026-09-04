use uuid::Uuid;

pub struct ExpenseId(Uuid);

impl ExpenseId {
    pub fn new() -> ExpenseId {
        ExpenseId(Uuid::new_v4())
    }
}

pub struct Expense {
    pub id: ExpenseId,
    pub description: String,
    pub amount: f64,
    pub category: String,
}

impl Expense {
    pub fn new(description: String, amount: f64, category: String) -> Expense {
        Expense {
            id: ExpenseId::new(),
            description,
            amount,
            category,
        }
    }
}

pub trait DisplayExpense {
    fn display(&self);
}

impl DisplayExpense for Expense {
    fn display(&self) {
        println!(
            "ID: {} | {} | ₦{} | {}",
            self.id.0, self.description, self.amount, self.category
        );
    }
}
