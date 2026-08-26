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

#[derive(Debug)]
enum ExpenseError {
    InvalidAmount,
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

    fn add_expense(
        &mut self,
        description: String,
        amount: f64,
        category: String,
    ) -> Result<(), ExpenseError> {
        if amount <= 0.0 {
            return Err(ExpenseError::InvalidAmount);
        }

        let id = ExpenseId(self.next_id);

        let expense = Expense::new(id, description, amount, category.clone());

        self.expenses.push(expense);

        let total = self.category_totals.entry(category).or_insert(0.0);

        *total += amount;

        self.next_id += 1;

        Ok(())
    }

    fn calculate_total(&self) -> f64 {
        let mut total = 0.0;

        for expense in &self.expenses {
            total += expense.amount;
        }

        total
    }
}

fn main() {
    let mut tracker = ExpenseTracker::new();

    let result = tracker.add_expense(String::from("Lunch"), 5000.0, String::from("Food"));

    match result {
        Ok(()) => println!("Expense added successfully."),
        Err(ExpenseError::InvalidAmount) => {
            println!("Invalid amount.");
        }
    }

    println!("Number of expenses: {}", tracker.expenses.len());

    for expense in &tracker.expenses {
        expense.display();
    }

    println!("{:?}", tracker.category_totals);
}
