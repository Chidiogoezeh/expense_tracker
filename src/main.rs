use std::io::{self, Write};

use expense_tracker::error::ExpenseError;
use expense_tracker::tracker::ExpenseTracker;

enum MenuOption {
    Add,
    Delete,
    Total,
    List,
    Exit,
    CategoryTotals,
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

fn read_id(prompt: &str) -> String {
    read_input(prompt)
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
    let id = read_id("Expense ID: ");

    match tracker.delete_expense(&id) {
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
