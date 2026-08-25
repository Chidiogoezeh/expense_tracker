struct Expence {
    id: ExpenceId,
    description: String,
    amount: f64,
    category: String,
}

struct Expences(u32);

enum MenuOption {
    Add,
    Delete,
    Total,
    List,
    Exit,
}

fn main() {
    println!("Let's work on wekk 3 project!");
}
