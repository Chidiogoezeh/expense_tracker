# Expense Tracker API

A RESTful **Expense Tracker API built with Rust**, developed throughout my **12-Week Rust Backend Learning Roadmap**.

The project evolved from a Rust CLI application into an authenticated backend API using **Axum, PostgreSQL, SQLx, JWT, Argon2, validation, structured logging, and production-style application architecture**.

## Current Features

### Authentication

- User registration and login
- Argon2 password hashing
- JWT authentication
- Protected routes
- User profile
- User-owned expenses

### Expenses

- Create expenses
- List expenses
- Delete expenses
- Calculate total expenses
- Calculate totals by category
- Expense validation
- Users can only access their own expenses

### Backend

- Rust + Axum
- PostgreSQL + SQLx
- Database migrations
- Async request handling with Tokio
- Centralized application errors
- Request validation
- Structured logging with `tracing`
- Authentication middleware
- Service and repository layers
- Application configuration
- Dependency injection

## Architecture

The project now follows a simple layered architecture:

```text
HTTP Request
     ↓
Middleware
     ↓
Handler
     ↓
Service
     ↓
Repository
     ↓
PostgreSQL
```

### Configuration

Application configuration is loaded from environment variables:

```text
.env
 ↓
Config
 ↓
AppState
```

Configuration includes:

```env
DATABASE_URL=...
JWT_SECRET=...
PORT=3000
```

### Dependency Injection

Application dependencies are created during startup and passed into the application state.

```text
main.rs
   ↓
Config
   ↓
Database Pool
   ↓
Repositories
   ↓
Services
   ↓
AppState
   ↓
Handlers
```

### Service Layer

Services contain application and business logic.

```text
auth/service.rs
expense/service.rs
```

Examples:

- Registering users
- Verifying passwords
- Creating JWTs
- Creating expenses
- Deleting expenses
- Calculating expense totals

### Repository Layer

Repositories contain database operations.

```text
auth/repository.rs
expense/repository.rs
```

They are responsible for executing SQL queries through SQLx.

## API

### Authentication

```text
POST /register
POST /login
GET  /profile
```

### Expenses

```text
POST   /expenses
GET    /expenses
DELETE /expenses/:id
GET    /expenses/total
GET    /expenses/categories
```

`/register` and `/login` are public.

`/profile` and all expense endpoints require:

```http
Authorization: Bearer <JWT>
```

### Create Expense

```http
POST /expenses
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

The authenticated user's ID is obtained from the JWT. The client does not provide `user_id`.

## Database

The application uses PostgreSQL with SQLx.

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

Each expense belongs to a user through `user_id`.

Database migrations are stored in:

```text
migrations/
```

Run them with:

```bash
sqlx migrate run
```

## Project Structure

```text
expense_tracker/
├── migrations/
├── src/
│   ├── auth/
│   │   ├── handler.rs
│   │   ├── repository.rs
│   │   ├── service.rs
│   │   └── mod.rs
│   ├── expense/
│   │   ├── handler.rs
│   │   ├── repository.rs
│   │   ├── service.rs
│   │   └── mod.rs
│   ├── config.rs
│   ├── error.rs
│   ├── middleware.rs
│   ├── state.rs
│   └── main.rs
├── .env
├── .gitignore
├── Cargo.toml
└── Cargo.lock
```

## How to Run

Create a PostgreSQL database and configure `.env`:

```env
DATABASE_URL=postgres://postgres:YOUR_PASSWORD@localhost/expense_tracker
JWT_SECRET=your-long-random-secret
PORT=3000
```

Run migrations:

```bash
sqlx migrate run
```

Start the API:

```bash
cargo run
```

The API runs at:

```text
http://127.0.0.1:3000
```

## Development Checks

```bash
cargo fmt --check
cargo check
cargo clippy
cargo test
```

## Week 10 Learning Goal

Week 10 focuses on moving the application toward a production-style architecture:

- Configuration
- Dependency injection
- Service layer
- Repository layer

The resulting request flow is:

```text
Request
   ↓
Middleware
   ↓
Handler
   ↓
Service
   ↓
Repository
   ↓
PostgreSQL
```

This keeps HTTP handling, business logic, database operations, and application configuration separated.

## Roadmap

Upcoming improvements include:

- Automated tests
- Docker and Docker Compose
- API documentation
- Deployment
- Further authentication and security improvements

---

**Part of my 12-Week Rust Backend Learning Roadmap.**
