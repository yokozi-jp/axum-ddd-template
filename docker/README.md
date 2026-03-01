# Docker Setup

## Development

Start PostgreSQL only:
```bash
docker compose -f docker/compose.yml up postgres -d
```

Start all services:
```bash
docker compose -f docker/compose.yml up -d
```

Stop services:
```bash
docker compose -f docker/compose.yml down
```

View logs:
```bash
docker compose -f docker/compose.yml logs -f
```

## Database Connection

- Host: localhost
- Port: 5432
- User: postgres
- Password: postgres
- Database: axum_ddd

Connection string:
```
postgres://postgres:postgres@localhost:5432/axum_ddd
```
