# Expense Tracker API

A RESTful **Expense Tracker API built with Rust**, evolved from a command-line application into a backend API as part of my **12-Week Rust Backend Learning Roadmap**.

The project started as a simple CLI application during Weeks 1–3 and was progressively expanded as I learned more Rust and backend development concepts through Week 8.

## Project Evolution

This project reflects my progression through the first eight weeks of the roadmap:

- **Week 1:** Rust fundamentals — variables, mutability, data types, functions, ownership, borrowing, references, and shadowing.

- **Week 2:** Structs, tuple structs, enums, `match`, `impl` blocks, and traits.

- **Week 3:** `Vec`, `HashMap`, `Option`, `Result`, and custom error handling.

- **Week 4:** Cargo, packages, crates, modules, visibility, and external crates.

- **Week 5:** Asynchronous Rust, futures, `async`/`await`, Tokio, and asynchronous tasks.

- **Week 6:** REST APIs, HTTP routes, handlers, JSON, and HTTP status codes.

- **Week 7:** PostgreSQL, SQLx, database connection pools, SQL queries, and database migrations.

- **Week 8:** Authentication — password hashing with Argon2, JWT authentication, user registration, login, authentication middleware, protected routes, and user-owned expenses.

The project's Git commit history shows this progression from the original **Expense Tracker CLI** to the current **authenticated PostgreSQL-backed Expense Tracker API**.

## Current Features

### Authentication

- User registration
- User login
- Password hashing with Argon2
- Password verification
- JWT creation
- JWT verification
- Authentication middleware
- Protected routes
- Authenticated user identification
- User profile endpoint
- Unique user email addresses
- User-owned expenses

### Expense Management

- Create expenses
- Delete expenses
- List expenses
- Calculate total expenses
- View expense totals by category
- Expenses associated with authenticated users
- Users can only access their own expenses

### Backend

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
- Asynchronous request handling

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
- Foreign keys
- Database migrations
- Database schema management

### Authentication

- Password hashing
- Argon2
- Password verification
- JWT
- JWT claims
- Bearer authentication
- JWT verification
- Authentication middleware
- Request extensions
- Protected routes
- User authentication
- User-owned resources

## API

The Expense Tracker is exposed through a REST API backed by PostgreSQL.

### Endpoints

#### Authentication

```text
POST /register
POST /login
GET  /profile
```

#### Expenses

```text
GET    /expenses
POST   /expenses
DELETE /expenses/:id
GET    /expenses/total
GET    /expenses/categories
```

`/register` and `/login` are public endpoints.

`/profile` and all expense endpoints require a valid JWT.

### Authentication Flow

```text
POST /register
      ↓
Validate input
      ↓
Hash password with Argon2
      ↓
Store user in PostgreSQL


POST /login
      ↓
Find user
      ↓
Verify password
      ↓
Create JWT
      ↓
Return JWT


Protected request
      ↓
Authorization: Bearer <JWT>
      ↓
JWT middleware
      ↓
Verify JWT
      ↓
Extract user ID
      ↓
Execute protected handler
```

### Register

```http
POST /register
Content-Type: application/json
```

Request:

```json
{
  "email": "chidi@example.com",
  "password": "password123"
}
```

Example response:

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "email": "chidi@example.com"
}
```

The password is hashed with Argon2 before being stored in PostgreSQL. The plaintext password is never stored.

### Login

```http
POST /login
Content-Type: application/json
```

Request:

```json
{
  "email": "chidi@example.com",
  "password": "password123"
}
```

Example response:

```json
{
  "token": "eyJ..."
}
```

The JWT is returned after the supplied password is successfully verified against the stored password hash.

### Authentication Header

Protected requests must include the JWT:

```http
Authorization: Bearer <JWT>
```

### Profile

```http
GET /profile
Authorization: Bearer <JWT>
```

Example response:

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "email": "chidi@example.com"
}
```

### Create an Expense

```http
POST /expenses
Authorization: Bearer <JWT>
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

The authenticated user's ID is obtained from the JWT by the authentication middleware and stored with the expense.

The client does **not** provide `user_id`.

### List Expenses

```http
GET /expenses
Authorization: Bearer <JWT>
```

Returns only the expenses belonging to the authenticated user.

### Get Total Expenses

```http
GET /expenses/total
Authorization: Bearer <JWT>
```

Example response:

```json
{
  "total": 5000
}
```

The total is calculated only from the authenticated user's expenses.

### Get Category Totals

```http
GET /expenses/categories
Authorization: Bearer <JWT>
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
Authorization: Bearer <JWT>
```

Replace `:id` with the expense UUID.

The authenticated user's ID is included when deleting the expense, ensuring that a user can only delete their own expense.

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
Authentication Middleware
    ↓
AppState
    ↓
SQLx Connection Pool
    ↓
PostgreSQL
```

### Database Schema

The database contains `users` and `expenses` tables.

```sql
CREATE TABLE users (
    id UUID PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE expenses (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    description TEXT NOT NULL,
    amount DOUBLE PRECISION NOT NULL,
    category TEXT NOT NULL
);
```

The `user_id` column connects each expense to its owner.

```text
users
  │
  ├── user_id ──→ expense
  ├── user_id ──→ expense
  └── user_id ──→ expense
```

This allows expense queries to be restricted to the authenticated user.

## Database Migrations

Database schema changes are managed using **SQLx migrations**.

Migration files are stored in:

```text
migrations/
```

Example:

```text
migrations/
└── 001_create_users_and_expenses.sql
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
│   └── 001_create_users_and_expenses.sql
│
├── src/
│   ├── auth.rs
│   ├── error.rs
│   ├── expense.rs
│   ├── main.rs
│   └── middleware.rs
│
├── .env
├── .gitignore
├── Cargo.toml
└── Cargo.lock
```

### Module Responsibilities

```text
main.rs
    Application state
    Database connection
    Route configuration
    Server startup

auth.rs
    Registration
    Login
    Profile
    Password hashing
    Password verification
    JWT creation

middleware.rs
    JWT extraction
    JWT verification
    Authenticated user identification

expense.rs
    Expense creation
    Expense retrieval
    Expense deletion
    Expense totals
    Category totals

error.rs
    Application error types
```

The project structure will continue to evolve as the application moves toward a more production-style backend architecture.

## How to Run

Make sure Rust, Cargo, PostgreSQL, and the SQLx CLI are installed.

Clone the repository:

```bash
git clone https://github.com/Chidiogoezeh/expense_tracker
```

Enter the project directory:

```bash
cd expense_tracker
```

Create a `.env` file:

```env
DATABASE_URL=postgres://postgres:YOUR_PASSWORD@localhost/expense_tracker
JWT_SECRET=your-long-random-secret-desires-LOL
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

### 1. Register

```http
POST http://127.0.0.1:3000/register
Content-Type: application/json
```

```json
{
  "email": "chidi@example.com",
  "password": "password123"
}
```

### 2. Login

```http
POST http://127.0.0.1:3000/login
Content-Type: application/json
```

```json
{
  "email": "chidi@example.com",
  "password": "password123"
}
```

Copy the returned JWT.

### 3. Access Profile

```http
GET http://127.0.0.1:3000/profile
Authorization: Bearer <JWT>
```

### 4. Create an Expense

```http
POST http://127.0.0.1:3000/expenses
Authorization: Bearer <JWT>
Content-Type: application/json
```

```json
{
  "description": "Lunch",
  "amount": 5000,
  "category": "Food"
}
```

### 5. Retrieve Expenses

```http
GET http://127.0.0.1:3000/expenses
Authorization: Bearer <JWT>
```

Requests to protected endpoints without a valid JWT should return:

```text
401 Unauthorized
```

## Learning Goal

The goal of this project is to apply the concepts learned throughout the first eight weeks of my Rust backend journey by continuously improving the same application.

Rather than building a separate project for every concept, I am using the Expense Tracker to demonstrate how a small Rust application can evolve from a **CLI program into an authenticated RESTful backend API with PostgreSQL persistence**.

The Week 8 implementation specifically demonstrates how authentication fits into a backend application:

```text
Registration
    ↓
Password Hashing
    ↓
PostgreSQL
    ↓
Login
    ↓
JWT
    ↓
Authentication Middleware
    ↓
Protected Routes
    ↓
User-Owned Expenses
```

## Future Improvements

As I progress through the remaining weeks of the roadmap, I plan to add:

- Request validation
- Structured logging
- Better application error handling
- Production-style project architecture
- Automated tests
- Docker and Docker Compose
- API documentation
- Deployment
- Additional authentication improvements

---

**Part of my 12-Week Rust Backend Learning Roadmap.**
