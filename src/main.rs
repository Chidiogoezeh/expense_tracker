use std::collections::HashMap;

struct Expense {
    id: ExpenseId,
    description: String,
    amount: f64,
    category: String,
}

struct ExpenseId(u32);

impl Expense {
    fn new(id: ExpenseId, description: String, amount: f64, category: String) -> Expense {
        Expense {
            id,
            description,
            amount,
            category,
        }
    }
}

trait DisplayExpense {
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

struct ExpenseTracker {
    expenses: Vec<Expense>,
    category_totals: HashMap<String, f64>,
    next_id: u32,
}

impl ExpenseTracker {
    fn new() -> ExpenseTracker {
        ExpenseTracker {
            expenses: Vec::new(),
            category_totals: HashMap::new(),
            next_id: 1,
        }
    }
}

fn main() {
    let mut tracker = ExpenseTracker::new();

    println!("Expense tracker created")
}
