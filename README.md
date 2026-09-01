# Expense Tracker CLI

A simple command-line expense tracker built with **Rust** as part of my 12-Week Rust Backend Learning Roadmap.

## Features

- Add an expense
- Delete an expense
- Calculate total expenses
- List all expenses
- View expenses by category
- Validate user input
- Handle errors with custom error types

## Rust Concepts Practiced

### Week 1

- Variables
- Mutability
- Data types
- Functions
- Ownership
- Borrowing
- References
- Shadowing

### Week 2

- Structs
- Tuple structs
- Enums
- `match`
- `impl` blocks
- Traits

### Week 3

- `Vec`
- `HashMap`
- `Option`
- `Result`
- Custom errors

## Project Structure

```text
expense_tracker/
├── Cargo.toml
└── src/
    └── main.rs
```

## How to Run

Make sure Rust and Cargo are installed.

Clone the repository:

```bash
git clone <https://github.com/Chidiogoezeh/expense_tracker>
```

Enter the project directory:

```bash
cd expense_tracker
```

Run the application:

```bash
cargo run
```

## Example

```text
=== Expense Tracker ===
1. Add expense
2. Delete expense
3. Calculate total
4. List expenses
5. Exit
6. Category totals

Choose an option: 1

Description: Lunch
Amount: 5000
Category: Food

Expense added successfully.
```

## Learning Goal

The purpose of this project is to practice Rust fundamentals and Week 3 data structures and error handling by building a small, functional CLI application.

## Future Improvements

Possible improvements for later weeks:

- Save expenses to a file
- Use PostgreSQL
- Build a REST API
- Add authentication
- Add automated tests
- Containerize with Docker
- Deploy the application

---

**Part of my 12-Week Rust Backend Learning Roadmap.**
