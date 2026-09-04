use std::collections::HashMap;

use crate::error::ExpenseError;
use crate::expense::{DisplayExpense, Expense};

pub struct ExpenseTracker {
    expenses: Vec<Expense>,
    category_totals: HashMap<String, f64>,
}

impl ExpenseTracker {
    pub fn new() -> ExpenseTracker {
        ExpenseTracker {
            expenses: Vec::new(),
            category_totals: HashMap::new(),
        }
    }

    pub fn add_expense(
        &mut self,
        description: String,
        amount: f64,
        category: String,
    ) -> Result<(), ExpenseError> {
        if amount <= 0.0 {
            return Err(ExpenseError::InvalidAmount);
        }

        let expense = Expense::new(description, amount, category.clone());

        self.expenses.push(expense);

        let total = self.category_totals.entry(category).or_insert(0.0);

        *total += amount;

        Ok(())
    }

    pub fn calculate_total(&self) -> f64 {
        let mut total = 0.0;

        for expense in &self.expenses {
            total += expense.amount;
        }

        total
    }

    pub fn delete_expense(&mut self, id: &str) -> Result<(), ExpenseError> {
        let position = self
            .expenses
            .iter()
            .position(|expense| expense.id.as_string() == id);

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

    pub fn list_expenses(&self) {
        if self.expenses.is_empty() {
            println!("No expenses found.");
            return;
        }

        for expense in &self.expenses {
            expense.display();
        }
    }

    pub fn show_category_totals(&self) {
        println!("\nCategory totals:");

        for (category, total) in &self.category_totals {
            println!("{}: ₦{}", category, total);
        }
    }
}
