# Expense Tracker API

A RESTful **Expense Tracker API built with Rust**, evolved from a command-line application into a backend API as part of my **12-Week Rust Backend Learning Roadmap**.

The project started as a simple CLI application during Weeks 1–3 and was progressively expanded as I learned more Rust and backend development concepts through Week 6.

## Project Evolution

This project reflects my progression through the first six weeks of the roadmap:

- **Week 1:** Rust fundamentals — variables, mutability, data types, functions, ownership, borrowing, references, and shadowing.
- **Week 2:** Structs, tuple structs, enums, `match`, `impl` blocks, and traits.
- **Week 3:** `Vec`, `HashMap`, `Option`, `Result`, and custom error handling.
- **Week 4:** Cargo, packages, crates, modules, visibility, and external crates.
- **Week 5:** Asynchronous Rust, futures, `async`/`await`, Tokio, and asynchronous tasks.
- **Week 6:** REST APIs, HTTP routes, handlers, JSON, and HTTP status codes.

The project's Git commit history shows this progression from the original **Expense Tracker CLI** to the current **Expense Tracker API**.

## Current Features

- Create expenses
- Delete expenses
- List expenses
- Calculate total expenses
- View expenses by category
- JSON request and response handling
- HTTP routing
- HTTP handlers
- HTTP status codes
- Input validation
- Error handling
- Asynchronous request handling

## Rust & Backend Concepts Practiced

### Rust Fundamentals

- Variables
- Mutability
- Data types
- Functions
- Ownership
- Borrowing
- References
- Shadowing

### Rust Data Modeling

- Structs
- Tuple structs
- Enums
- `match`
- `impl` blocks
- Traits

### Collections & Error Handling

- `Vec`
- `HashMap`
- `Option`
- `Result`
- Custom error types

### Project & Module Organization

- Cargo
- Packages
- Crates
- Modules
- Visibility
- External crates

### Asynchronous Rust

- Futures
- `async`
- `await`
- Tokio
- Asynchronous tasks

### REST API Development

- HTTP methods
- Routes
- Handlers
- JSON serialization
- JSON deserialization
- HTTP status codes
- Request handling
- Response handling

## API

The Expense Tracker is now exposed through a REST API instead of only running as a command-line application.

Example endpoints:

```text
GET    /expenses
POST   /expenses
DELETE /expenses/:id
GET    /expenses/total
GET    /expenses/category/:category
```

Example JSON request:

```json
{
  "description": "Lunch",
  "amount": 5000,
  "category": "Food"
}
```

Example JSON response:

```json
{
  "id": 1,
  "description": "Lunch",
  "amount": 5000,
  "category": "Food"
}
```

## Project Structure

```text
expense_tracker/
├── Cargo.toml
└── src/
    └── main.rs
```

> The project structure will continue to evolve as the application moves toward a more production-style backend architecture.

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

The API will be available at:

```text
http://127.0.0.1:3000
```

## Example

Create an expense:

```http
POST /expenses
Content-Type: application/json
```

```json
{
  "description": "Lunch",
  "amount": 5000,
  "category": "Food"
}
```

The API returns the appropriate HTTP status code and JSON response.

## Learning Goal

The goal of this project is to apply the concepts learned throughout the first six weeks of my Rust backend journey by continuously improving the same application.

Rather than building a separate project for every concept, I am using the Expense Tracker to demonstrate how a small Rust application can evolve from a **CLI program into a RESTful backend API**.

## Future Improvements

As I progress through the remaining weeks of the roadmap, I plan to add:

- PostgreSQL database integration
- SQLx
- Authentication and authorization
- Request validation
- Structured logging
- Production-style project architecture
- Automated tests
- Docker and Docker Compose
- API documentation
- Deployment

---

**Part of my 12-Week Rust Backend Learning Roadmap.**
