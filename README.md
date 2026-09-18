# Expense Tracker API

A RESTful **Expense Tracker API built with Rust**, evolved from a command-line application into a backend API as part of my **12-Week Rust Backend Learning Roadmap**.

The project started as a simple CLI application during Weeks 1–3 and was progressively expanded as I learned more Rust and backend development concepts through Week 7.

## Project Evolution

This project reflects my progression through the first seven weeks of the roadmap:

- **Week 1:** Rust fundamentals — variables, mutability, data types, functions, ownership, borrowing, references, and shadowing.
- **Week 2:** Structs, tuple structs, enums, `match`, `impl` blocks, and traits.
- **Week 3:** `Vec`, `HashMap`, `Option`, `Result`, and custom error handling.
- **Week 4:** Cargo, packages, crates, modules, visibility, and external crates.
- **Week 5:** Asynchronous Rust, futures, `async`/`await`, Tokio, and asynchronous tasks.
- **Week 6:** REST APIs, HTTP routes, handlers, JSON, and HTTP status codes.
- **Week 7:** PostgreSQL, SQLx, database connection pools, SQL queries, and database migrations.

The project's Git commit history shows this progression from the original **Expense Tracker CLI** to the current **PostgreSQL-backed Expense Tracker API**.

## Current Features

- Create expenses
- Delete expenses
- List expenses
- Calculate total expenses
- View expense totals by category
- PostgreSQL database persistence
- SQLx database integration
- Database connection pooling
- Database migrations
- SQL queries
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

### Database Development

- PostgreSQL
- SQLx
- PostgreSQL connection pools
- Database connections
- SQL queries
- `SELECT`
- `INSERT`
- `DELETE`
- Aggregate queries
- `SUM`
- `GROUP BY`
- Database migrations
- Database schema management

## API

The Expense Tracker is exposed through a REST API backed by PostgreSQL.

### Endpoints

```text
GET    /expenses
POST   /expenses
DELETE /expenses/:id
GET    /expenses/total
GET    /expenses/categories
```

### Create an Expense

```http
POST /expenses
Content-Type: application/json
```

Request:

```json
{
  "description": "Lunch",
  "amount": 5000,
  "category": "Food"
}
```

Example response:

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "description": "Lunch",
  "amount": 5000,
  "category": "Food"
}
```

### List Expenses

```http
GET /expenses
```

Returns all expenses stored in PostgreSQL.

### Get Total Expenses

```http
GET /expenses/total
```

Example response:

```json
{
  "total": 5000
}
```

### Get Category Totals

```http
GET /expenses/categories
```

Example response:

```json
[
  {
    "category": "Food",
    "total": 5000
  }
]
```

### Delete an Expense

```http
DELETE /expenses/:id
```

Replace `:id` with the expense UUID.

Successful deletion returns:

```text
204 No Content
```

## Database

The application uses **PostgreSQL** for persistent data storage and **SQLx** for database communication.

A SQLx connection pool is created when the application starts and shared with the API handlers.

```text
Axum API
    ↓
SQLx Connection Pool
    ↓
PostgreSQL
```

This replaces the earlier in-memory `Vec<Expense>` and `HashMap` storage.

### Database Schema

The current database contains an `expenses` table:

```sql
CREATE TABLE expenses (
    id UUID PRIMARY KEY,
    description TEXT NOT NULL,
    amount DOUBLE PRECISION NOT NULL,
    category TEXT NOT NULL
);
```

## Database Migrations

Database schema changes are managed using **SQLx migrations**.

Migration files are stored in:

```text
migrations/
```

Example:

```text
migrations/
└── 20260914xxxxxx_create_expenses.sql
```

Run migrations with:

```bash
sqlx migrate run
```

Migrations allow the database structure to be version-controlled and reproduced consistently across environments.

## Project Structure

```text
expense_tracker/
├── migrations/
│   └── 20260914xxxxxx_create_expenses.sql
│
├── src/
│   ├── error.rs
│   ├── expense.rs
│   └── main.rs
│
├── Cargo.toml
└── Cargo.lock
```

> The project structure will continue to evolve as the application moves toward a more production-style backend architecture.

## How to Run

Make sure Rust, Cargo, and PostgreSQL are installed.

Clone the repository:

```bash
git clone https://github.com/Chidiogoezeh/expense_tracker
```

Enter the project directory:

```bash
cd expense_tracker
```

Set the database connection string.

On Windows Command Prompt:

```cmd
set DATABASE_URL=postgres://postgres:YOUR_PASSWORD@localhost/expense_tracker
```

Run the database migrations:

```bash
sqlx migrate run
```

Start the application:

```bash
cargo run
```

The API will be available at:

```text
http://127.0.0.1:3000
```

## Testing

The API endpoints can be tested using Postman.

Example:

```http
POST http://127.0.0.1:3000/expenses
```

```json
{
  "description": "Lunch",
  "amount": 5000,
  "category": "Food"
}
```

The created expense is stored in PostgreSQL and can then be retrieved using:

```http
GET http://127.0.0.1:3000/expenses
```

## Learning Goal

The goal of this project is to apply the concepts learned throughout the first seven weeks of my Rust backend journey by continuously improving the same application.

Rather than building a separate project for every concept, I am using the Expense Tracker to demonstrate how a small Rust application can evolve from a **CLI program into a RESTful backend API with persistent PostgreSQL storage**.

## Future Improvements

As I progress through the remaining weeks of the roadmap, I plan to add:

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
