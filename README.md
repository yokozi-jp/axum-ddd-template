# Axum DDD Template

A Domain-Driven Design template using Axum framework with Package by Feature architecture.

## Features

- **User Management** (PostgreSQL Repository)
- **Task Management** (PostgreSQL Repository)
- Clean Architecture + DDD + Hexagonal Architecture
- Package by Feature structure

## Prerequisites

- Rust 1.85+
- Docker & Docker Compose
- PostgreSQL 16
- [sqlx-cli](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli) (optional, for migration management)

```bash
cargo install sqlx-cli --no-default-features --features postgres
```

## Setup

1. Clone the repository

2. Copy environment file:
```bash
cp .env.example .env
```

3. Start PostgreSQL:
```bash
docker compose -f docker/compose.yml up postgres -d
```

4. Create database:
```bash
docker exec docker-postgres-1 psql -U postgres -c "CREATE DATABASE axum_ddd;"
```

Migrations run automatically on `cargo run`.

## Running

```bash
cargo run
```

Server will start on `http://localhost:3000`

## API Examples

### Health Check
```bash
curl http://localhost:3000/health
```

### User Management

**Create User**
```bash
curl -X POST http://localhost:3000/users \
  -H "Content-Type: application/json" \
  -d '{"name":"Alice","email":"alice@example.com"}'
```

**List Users**
```bash
curl http://localhost:3000/users
```

**Get User**
```bash
curl http://localhost:3000/users/{id}
```

**Update User**
```bash
curl -X PUT http://localhost:3000/users/{id} \
  -H "Content-Type: application/json" \
  -d '{"name":"Bob","email":"bob@example.com"}'
```

**Delete User**
```bash
curl -X DELETE http://localhost:3000/users/{id}
```

### Task Management

**Create Task**
```bash
curl -X POST http://localhost:3000/tasks \
  -H "Content-Type: application/json" \
  -d '{"user_id":"{user_id}","title":"Buy milk","description":"Get 2 liters"}'
```

**List All Tasks**
```bash
curl http://localhost:3000/tasks
```

**List Tasks by User**
```bash
curl "http://localhost:3000/tasks?user_id={user_id}"
```

**Get Task**
```bash
curl http://localhost:3000/tasks/{id}
```

**Complete Task**
```bash
curl -X PATCH http://localhost:3000/tasks/{id}/complete
```

**Delete Task**
```bash
curl -X DELETE http://localhost:3000/tasks/{id}
```

## Development

### Build & Check

```bash
# Check compilation
cargo check

# Run linter
cargo clippy

# Run linter (strict, treat warnings as errors)
cargo clippy -- -D warnings

# Format code
cargo fmt

# Run tests
cargo test
```

### Database & Migrations

```bash
# Start PostgreSQL
docker compose -f docker/compose.yml up postgres -d

# Stop PostgreSQL
docker compose -f docker/compose.yml down

# Show migration status
sqlx migrate info

# Run pending migrations manually
sqlx migrate run

# Revert last migration
sqlx migrate revert

# Create a new migration
sqlx migrate add <name>
```

### Process Management

```bash
# Kill running server
pkill -f axum-ddd-template

# Run with debug logging
RUST_LOG=debug cargo run

# Run with specific log filter
RUST_LOG=axum_ddd_template=debug,sqlx=warn cargo run
```

### Environment Variables

| Variable | Default | Description |
|---|---|---|
| `DATABASE_URL` | *(required)* | PostgreSQL connection URL |
| `SERVER_HOST` | `0.0.0.0` | Server bind address |
| `SERVER_PORT` | `3000` | Server port |
| `DB_MAX_CONNECTIONS` | `10` | Max DB pool connections |
| `DB_MIN_CONNECTIONS` | `2` | Min DB pool connections |
| `DB_ACQUIRE_TIMEOUT_SECS` | `30` | Connection acquire timeout |
| `DB_IDLE_TIMEOUT_SECS` | `600` | Idle connection timeout |

## Architecture

```
src/
├── features/          # Package by Feature
│   ├── user/
│   │   ├── domain/        # Entity, value objects, repository port
│   │   ├── application/   # Use cases (create, get, update, delete)
│   │   └── infrastructure/ # HTTP handlers, PostgreSQL repository
│   └── task/
│       ├── domain/
│       ├── application/   # Use cases (create, get, complete, delete)
│       └── infrastructure/
└── shared/            # Cross-feature shared code
    ├── domain/        # Entity trait, DomainError, Email value object
    └── infrastructure/ # Config, DB pool, HTTP error mapping
```

## License

Apache-2.0
