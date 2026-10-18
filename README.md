# Personal Finance API

A RESTful personal finance API built with Rust as part of my **12-Week Rust Backend Learning Roadmap**.

The project is evolving from an expense tracker into a backend for managing income, expenses, transactions, budgets, and financial summaries using **Rust, Axum, PostgreSQL, SQLx, JWT, and Argon2**.

## Current Features

### Authentication and Security

- User registration and login
- Argon2 password hashing
- JWT authentication
- Protected endpoints
- User profile
- User-scoped financial data access

### Transactions

- Unified transaction model for income and expenses
- Create, list, retrieve, update, and delete transactions
- Transaction categories
- Transaction types: `INCOME` and `EXPENSE`
- Validation of transaction data
- User ownership enforced through authenticated requests

### Dashboard and Reports

- Total income
- Total expenses
- Net cash flow
- Spending summaries by category
- Date-filtered financial reports

### Budgets

- Create and manage monthly budgets
- Set spending limits by category
- Track spending against budget limits

### Backend

- Rust and Axum
- PostgreSQL and SQLx
- Database migrations
- Asynchronous request handling with Tokio
- Centralized application errors
- Request validation
- Structured logging with `tracing`
- Authentication middleware
- Service and repository layers
- Environment-based configuration
- Shared application state and dependency injection

> **Implementation status:** Update the feature lists above to match the endpoints and tests currently working in your codebase. Features that are still being developed should be moved to the roadmap below.

## Architecture

The application follows a layered architecture:

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

- **Handlers** process HTTP requests and responses.
- **Services** contain business logic.
- **Repositories** execute database operations through SQLx.
- **Middleware** handles cross-cutting request concerns, including authentication.
- **AppState** provides shared application dependencies.

## Data Model

The application uses a unified `transactions` table instead of separate income and expense tables. An expense is a transaction whose type is `EXPENSE`; income uses `INCOME`.

The core entities are:

- **Users:** Account identity and authentication data.
- **Transactions:** Income and expense records, including amount, category, currency, and transaction date.
- **Budgets:** Monthly spending limits associated with a user, category, and currency.

Financial records are associated with their owner. Queries for private data must be scoped to the authenticated user's ID.

## API

### Authentication

| Method | Endpoint    | Purpose                                   |
| ------ | ----------- | ----------------------------------------- |
| `POST` | `/register` | Register an account                       |
| `POST` | `/login`    | Authenticate and obtain a JWT             |
| `GET`  | `/profile`  | Retrieve the authenticated user's profile |

### Transactions

| Method   | Endpoint             | Purpose                |
| -------- | -------------------- | ---------------------- |
| `POST`   | `/transactions`      | Create a transaction   |
| `GET`    | `/transactions`      | List transactions      |
| `GET`    | `/transactions/{id}` | Retrieve a transaction |
| `PATCH`  | `/transactions/{id}` | Update a transaction   |
| `DELETE` | `/transactions/{id}` | Delete a transaction   |

Transaction listing is intended to support filtering by type, category, and date range, with pagination.

### Dashboard and Reports

| Method | Endpoint              | Purpose                                       |
| ------ | --------------------- | --------------------------------------------- |
| `GET`  | `/dashboard/summary`  | Summarize income, expenses, and net cash flow |
| `GET`  | `/reports/categories` | Summarize expenses by category                |

### Budgets

| Method   | Endpoint        | Purpose         |
| -------- | --------------- | --------------- |
| `GET`    | `/budgets`      | List budgets    |
| `POST`   | `/budgets`      | Create a budget |
| `PATCH`  | `/budgets/{id}` | Update a budget |
| `DELETE` | `/budgets/{id}` | Delete a budget |

All endpoints except registration and login require a valid bearer token.

```http
Authorization: Bearer <JWT>
```

## Configuration

The application reads its configuration from environment variables.

```env
DATABASE_URL=postgres://postgres:YOUR_PASSWORD@localhost:5432/expense_tracker
JWT_SECRET=your-long-random-secret__generate using random key generator
PORT=3000
```

Keep `.env` out of version control. Use a strong secret and environment-specific configuration outside local development.

## Database Migrations

Database migrations are stored in:

```text
migrations/
```

Apply migrations using SQLx:

```bash
sqlx migrate run
```

Set `DATABASE_URL` before running the migration command. Apply migrations against the intended development database, and back up data before making destructive schema changes.

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
│   ├── transaction/
│   │   ├── handler.rs
│   │   ├── repository.rs
│   │   ├── service.rs
│   │   ├── model.rs
│   │   └── mod.rs
│   ├── budget/
│   │   ├── handler.rs
│   │   ├── repository.rs
│   │   ├── service.rs
│   │   └── mod.rs
│   ├── dashboard/
│   │   ├── handler.rs
│   │   ├── repository.rs
│   │   ├── service.rs
│   │   └── mod.rs
│   ├── config.rs
│   ├── error.rs
│   ├── middleware.rs
│   ├── state.rs
│   └── main.rs
├── .env.example
├── .gitignore
├── Cargo.toml
└── Cargo.lock
```

## Running Locally

### Prerequisites

- Rust and Cargo
- PostgreSQL
- SQLx CLI

Install the SQLx CLI if needed:

```bash
cargo install sqlx-cli --no-default-features --features postgres,rustls
```

Create a PostgreSQL database, configure the environment variables, and apply migrations:

```bash
sqlx migrate run
cargo run
```

The API is expected at:

```text
http://127.0.0.1:3000
```

### Development Checks

Run these checks before committing changes:

```bash
cargo fmt --check
cargo check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

## Remaining Work

The following items are **not yet completed** and remain part of the production-readiness roadmap:

- Complete OpenAPI documentation and Swagger UI
- Dockerfile and Docker Compose configuration
- Health and readiness checks
- Reliable startup, database connectivity checks, and graceful shutdown
- End-to-end verification of the Docker workflow
- GitHub Actions CI for formatting, linting, tests, and builds
- Deployment to a test environment
- Deployment and database migration documentation

## Roadmap

The goal is to build a maintainable, secure personal finance backend with reliable transaction management, budgeting, financial summaries, automated testing, and reproducible deployment.

Bank integrations, payment processing, AI financial advice, OCR, and microservices are outside the current scope.
