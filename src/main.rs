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
    ExpenseNotFound,
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

    fn delete_expense(&mut self, id: u32) -> Result<(), ExpenseError> {
        let position = self.expenses.iter().position(|expense| expense.id.0 == id);

        match position {
            Some(index) => {
                let expense = self.expenses.remove(index);

                if let Some(total) = self.category_totals.get_mut(&expense.category) {
                    *total -= expense.amount;

                    if *total <= 0.0 {
                        self.category_totals.remove(&expense.category);
                    }
                }

                Ok(())
            }

            None => Err(ExpenseError::ExpenseNotFound),
        }
    }

    fn list_expenses(&self) {
        if self.expenses.is_empty() {
            println!("No expenses found.");
            return;
        }

        for expense in &self.expenses {
            expense.display();
        }
    }

    fn show_category_totals(&self) {
        println!("\nCategory totals:");

        for (category, total) in &self.category_totals {
            println!("{}: ₦{}", category, total);
        }
    }
}

fn print_menu() {
    println!("\n=== Expense Tracker ===");
    println!("1. Add expense");
    println!("2. Delete expense");
    println!("3. Calculate total");
    println!("4. List expenses");
    println!("5. Exit");
    println!("6. Category totals");
}

fn main() {
    let mut tracker = ExpenseTracker::new();

    loop {
        print_menu();
    }
}
