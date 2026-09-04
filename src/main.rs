use std::collections::HashMap;
use std::io::{self, Write};

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

enum MenuOption {
    Add,
    Delete,
    Total,
    List,
    Exit,
    CategoryTotals,
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

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}

fn read_u32(prompt: &str) -> Result<u32, String> {
    let input = read_input(prompt);

    input
        .parse::<u32>()
        .map_err(|_| String::from("Please enter a valid number."))
}

fn read_amount(prompt: &str) -> Result<f64, String> {
    let input = read_input(prompt);

    input
        .parse::<f64>()
        .map_err(|_| String::from("Please enter a valid amount."))
}

fn parse_menu_option(choice: u32) -> Option<MenuOption> {
    match choice {
        1 => Some(MenuOption::Add),
        2 => Some(MenuOption::Delete),
        3 => Some(MenuOption::Total),
        4 => Some(MenuOption::List),
        5 => Some(MenuOption::Exit),
        6 => Some(MenuOption::CategoryTotals),
        _ => None,
    }
}

fn handle_add(tracker: &mut ExpenseTracker) {
    let description = read_input("Description: ");

    let amount = match read_amount("Amount: ") {
        Ok(amount) => amount,
        Err(error) => {
            println!("{}", error);
            return;
        }
    };

    let category = read_input("Category: ");

    match tracker.add_expense(description, amount, category) {
        Ok(()) => {
            println!("Expense added successfully.");
        }

        Err(ExpenseError::InvalidAmount) => {
            println!("Amount must be greater than zero.");
        }

        Err(ExpenseError::ExpenseNotFound) => {
            println!("Expense not found.");
        }
    }
}

fn handle_delete(tracker: &mut ExpenseTracker) {
    let id = match read_u32("Expense ID: ") {
        Ok(id) => id,
        Err(error) => {
            println!("{}", error);
            return;
        }
    };

    match tracker.delete_expense(id) {
        Ok(()) => {
            println!("Expense deleted successfully.");
        }

        Err(ExpenseError::ExpenseNotFound) => {
            println!("Expense not found.");
        }

        Err(ExpenseError::InvalidAmount) => {
            println!("Invalid amount.");
        }
    }
}

fn main() {
    let mut tracker = ExpenseTracker::new();

    loop {
        print_menu();

        let choice = match read_u32("Choose an option: ") {
            Ok(choice) => choice,
            Err(error) => {
                println!("{}", error);
                continue;
            }
        };

        let option = match parse_menu_option(choice) {
            Some(option) => option,
            None => {
                println!("Invalid option.");
                continue;
            }
        };

        match option {
            MenuOption::Add => {
                handle_add(&mut tracker);
            }

            MenuOption::Delete => {
                handle_delete(&mut tracker);
            }

            MenuOption::Total => {
                let total = tracker.calculate_total();
                println!("Total expenses: ₦{}", total);
            }

            MenuOption::List => {
                tracker.list_expenses();
            }

            MenuOption::Exit => {
                println!("Goodbye!");
                break;
            }

            MenuOption::CategoryTotals => {
                tracker.show_category_totals();
            }
        }
    }
}
