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

enum MenuOption {
    Add,
    Delete,
    Total,
    List,
    Exit,
}

fn main() {
    let expense = Expense::new(
        ExpenseId(1),
        String::from("Lunch"),
        5000.0,
        String::from("Food"),
    );

    println!("{}", expense.description);
    println!("{}", expense.amount);
}
