<!-- Parent: ../AGENTS.md -->
# PostgreSQL Init Scripts

## Purpose

Database initialization scripts for PostgreSQL container. Run automatically on first container startup to configure database permissions and extensions.

## Key Files

| File | Purpose |
|------|---------|
| `01-grant-test-permissions.sh` | Grants necessary permissions for running integration tests |

## For AI Agents

### How Init Scripts Work

1. Scripts are mounted to `/docker-entrypoint-initdb.d/` in postgres container
2. PostgreSQL runs scripts in alphabetical order on first startup
3. Only executes if data directory is empty (fresh container)

### Script Structure

```bash
#!/bin/bash
set -e  # Exit on error

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
    -- SQL commands here
    CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
EOSQL
```

### Writing New Init Scripts

**Naming convention:**
- `XX-description.sh` where XX is numeric prefix
- Lower numbers run first (01, 02, 03...)
- Use descriptive names

**Requirements:**
- Make executable: `chmod +x script.sh`
- Include `#!/bin/bash` shebang
- Use `set -e` for error propagation
- Use `ON_ERROR_STOP=1` for SQL errors

### Current Script: `01-grant-test-permissions.sh`

Grants CREATE permission on database for test user. Required for SQLx test fixtures which create temporary schemas.

### Common Use Cases

**Add PostgreSQL extension:**
```bash
#!/bin/bash
set -e

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
    CREATE EXTENSION IF NOT EXISTS "pgcrypto";
    CREATE EXTENSION IF NOT EXISTS "pg_trgm";
EOSQL
```

**Create test database:**
```bash
#!/bin/bash
set -e

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" <<-EOSQL
    CREATE DATABASE voicechat_test;
    GRANT ALL PRIVILEGES ON DATABASE voicechat_test TO $POSTGRES_USER;
EOSQL
```

### Debugging

**Scripts not running:**
1. Check file is executable: `ls -la init-scripts/`
2. Verify shebang is correct
3. Check postgres logs: `docker compose logs postgres`
4. Ensure data volume is clean (scripts only run on first init)

**Re-run scripts:**
```bash
# Remove data volume to reset
docker compose down -v
docker compose up -d
```

### Mount Configuration

In `docker-compose.yml`:
```yaml
postgres:
  volumes:
    - ../docker/init-scripts:/docker-entrypoint-initdb.d:ro
```

## Dependencies

- PostgreSQL container (official image)
- Mounted via Docker Compose from `infra/compose/`
