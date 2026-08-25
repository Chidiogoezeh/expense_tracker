struct Expense {
    id: ExpenseId,
    description: String,
    amount: f64,
    category: String,
}

struct ExpenseId(u32);

enum MenuOption {
    Add,
    Delete,
    Total,
    List,
    Exit,
}

#[derive(Debug)]
enum ExpenseError {
    ExpenseNotFound,
    InvalidAmount,
}

// Simple example
fn add_expense(amount: f64) -> Result<(), ExpenseError> {
    if amount <= 0.0 {
        return Err(ExpenseError::InvalidAmount);
    }

    println!("Expense is valid");

    Ok(())
}

fn main() {
    let result = add_expense(5000.0);

    match result {
        Ok(()) => println!("Expense added successfully."),

        Err(ExpenseError::InvalidAmount) => {
            println!("Amount must be greater than zero.");
        }

        Err(ExpenseError::ExpenseNotFound) => {
            println!("Expense was not found")
        }
    }
}
