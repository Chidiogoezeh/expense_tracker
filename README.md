# Personal Finance API

A RESTful personal finance API built with Rust as part of my 12-Week Rust Backend Learning Roadmap. It supports user authentication, income and expense transactions, monthly budgets, and financial reports.

## 1. Technology Stack

- **Language:** Rust
- **HTTP:** Axum and Tokio
- **Database:** PostgreSQL and SQLx
- **Authentication:** JWT and Argon2
- **Validation:** Validator
- **Documentation:** OpenAPI and Swagger UI
- **Testing:** Rust tests and Axum integration tests
- **Deployment:** Docker and Docker Compose

The application uses handler, service, and repository layers to separate HTTP handling, business logic, and database operations.

## 2. Prerequisites

Install:

- [Rust](https://www.rust-lang.org/tools/install) and Cargo
- [Docker Desktop](https://www.docker.com/products/docker-desktop/)
- [Git](https://git-scm.com/)
- [pgAdmin 4](https://www.pgadmin.org/download/) (optional, for managing PostgreSQL)

For local PostgreSQL development without Docker, install PostgreSQL separately.

## 3. Environment Variables

Copy `.env.example` to `.env` and update the values for your local environment.

```powershell
Copy-Item .env.example .env
```

The application configuration uses these variables:

| Variable            | Purpose                                      |
| ------------------- | -------------------------------------------- |
| `DATABASE_URL`      | PostgreSQL connection string                 |
| `JWT_SECRET`        | Secret used to sign JWTs                     |
| `PORT`              | API port; defaults to `3000`                 |
| `TEST_DATABASE_URL` | Dedicated database used by integration tests |

Example local configuration:

```env
DATABASE_URL=postgres://postgres:CHANGE_ME@localhost:5432/expense_tracker
JWT_SECRET=REPLACE_WITH_A_RANDOM_SECRET
PORT=3000
TEST_DATABASE_URL=postgres://postgres:CHANGE_ME@localhost:5432/expense_tracker_test
```

These are placeholders, not usable credentials. Match the database names, usernames, passwords, and ports to your setup. Keep `.env` out of Git and never commit real secrets or production connection strings.

## 4. Local Development and Test Database

Clone the repository and enter the project directory:

```powershell
git clone https://github.com/Chidiogoezeh/expense_tracker.git
cd expense_tracker
```

Create the development and test databases in PostgreSQL if they do not already exist:

```sql
CREATE DATABASE expense_tracker;
CREATE DATABASE expense_tracker_test;
```

Configure `.env` with the correct connection strings. Ensure `TEST_DATABASE_URL` points to the dedicated test database, not your development or production database.

Start the API:

```powershell
cargo run
```

The application applies its SQLx migrations during startup. When it starts successfully, the API is available at `http://localhost:3000`.

## 5. Database Migrations

Migration files are stored in `migrations/` and are version-controlled with the source code.

The application runs pending migrations during startup. For manual migration management, install the SQLx CLI:

```powershell
cargo install sqlx-cli --no-default-features --features postgres,rustls
```

After configuring `DATABASE_URL`, run:

```powershell
sqlx migrate run
```

Apply migrations only to the intended database. Back up important data before deploying schema changes, and review migrations for destructive operations.

## 6. Docker Compose

With Docker Desktop running and the project environment configured, build and start the services:

```powershell
docker compose up --build -d
docker compose ps
```

View API logs:

```powershell
docker compose logs --tail=100 api
docker compose logs -f api
```

View database logs:

```powershell
docker compose logs --tail=100 db
```

Stop the services:

```powershell
docker compose down
```

`docker compose down` preserves named volumes. **Do not use `docker compose down -v` unless you intend to delete the associated persistent volumes and their data.**

The Compose database service is named `db`. Inside the Compose network, the API should connect to PostgreSQL using the service hostname and container port, rather than `localhost` and the host-mapped port.

## 7. Formatting, Linting, and Tests

Run these checks from the repository root before committing:

```powershell
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

Integration tests require `TEST_DATABASE_URL` to point to the dedicated test database. The test setup checks the database name before applying migrations. Never point it at a database containing important development or production data.

## 8. API Usage and Swagger UI

### Authentication

| Method | Endpoint    | Purpose                              |
| ------ | ----------- | ------------------------------------ |
| `POST` | `/register` | Register a user                      |
| `POST` | `/login`    | Log in and obtain a JWT              |
| `GET`  | `/profile`  | Get the authenticated user's profile |
| `GET`  | `/health`   | Check API and database health        |

### Transactions

| Method   | Endpoint             | Purpose                |
| -------- | -------------------- | ---------------------- |
| `POST`   | `/transactions`      | Create a transaction   |
| `GET`    | `/transactions`      | List transactions      |
| `GET`    | `/transactions/{id}` | Retrieve a transaction |
| `PATCH`  | `/transactions/{id}` | Update a transaction   |
| `DELETE` | `/transactions/{id}` | Delete a transaction   |

The transaction list supports these query parameters:

- `type`: `INCOME` or `EXPENSE`
- `category`: category filter, up to 50 characters
- `from` and `to`: inclusive date filters in `YYYY-MM-DD` format
- `page`: page number, defaults to `1`
- `limit`: page size, defaults to `20` and has a maximum of `100`

Example:

```text
GET /transactions?type=EXPENSE&from=2026-01-01&to=2026-01-31&page=1&limit=20
```

### Dashboard and Reports

| Method | Endpoint              | Purpose                                       |
| ------ | --------------------- | --------------------------------------------- |
| `GET`  | `/dashboard/summary`  | Summarize income, expenses, and net cash flow |
| `GET`  | `/reports/categories` | Summarize expenses by category                |

Both report endpoints accept `from` and `to` dates and an optional `currency` parameter.

### Budgets

| Method   | Endpoint        | Purpose         |
| -------- | --------------- | --------------- |
| `GET`    | `/budgets`      | List budgets    |
| `POST`   | `/budgets`      | Create a budget |
| `PATCH`  | `/budgets/{id}` | Update a budget |
| `DELETE` | `/budgets/{id}` | Delete a budget |

Registration, login, and health are public. All other application endpoints require a valid JWT:

```http
Authorization: Bearer YOUR_JWT
```

OpenAPI JSON:

`http://localhost:3000/api-docs/openapi.json`

Swagger UI:

`http://localhost:3000/swagger-ui/`

Use Swagger UI to inspect request and response schemas, try public endpoints, and authorize protected requests with a token obtained from login.

## 9. GitHub Actions

GitHub Actions workflows are stored in `.github/workflows/`.

The CI checks should run formatting, Clippy, and tests on pushes and pull requests. Integration tests must have access to an isolated PostgreSQL test database and a valid test configuration.

Check the **Actions** tab in GitHub for the result of each workflow. Keep required checks passing before merging changes.

## 10. Test Deployment and Database Operations

### Deployment configuration

Use a separate test environment with its own database and secrets. Configure `DATABASE_URL`, `JWT_SECRET`, and `PORT` through the deployment environment rather than committing them to the repository.

Build and start the test deployment using the project's Docker Compose configuration:

```powershell
docker compose up --build -d
docker compose ps
docker compose logs --tail=100 api
```

Verify `GET /health`, inspect the API logs, and test registration, login, and protected endpoints before treating the deployment as ready.

### Migrations

The application applies pending migrations during startup. Review migration files and back up the test database before deploying schema changes. If a migration fails, inspect the logs and resolve the database issue before continuing.

### Backup and restore

For a PostgreSQL database running in the Compose `db` service, use PostgreSQL's backup tools. Replace `DB_USER` and `DB_NAME` with the values configured for that database.

Create a custom-format backup from PowerShell:

```powershell
cmd /c "docker compose exec -T db pg_dump -U DB_USER -d DB_NAME -Fc > backup.dump"
```

Restore into the intended existing database after confirming the target and backup. **Restoring with `--clean` can drop existing database objects.**

```powershell
cmd /c "docker compose exec -T db pg_restore -U DB_USER -d DB_NAME --clean --if-exists < backup.dump"
```

Replace `DB_USER` and `DB_NAME` before running either command. Keep backups in a secure location, restrict access, and periodically verify that backups can be restored. Never commit backups or real financial records to Git.

---

This project is a learning-focused backend. Financial institution integrations, payment processing, and production financial advice are outside its current scope.
