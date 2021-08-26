# Load environment variables
set dotenv-load := true

# Launch database instance inside a Docker container
db-up:
    sh ./scripts/init_db.sh

# Run migrations against the current PostgreSQL instance
db-migrate $SKIP_DOCKER="true":
    sh ./scripts/init_db.sh

# Stop running database instance and remove its container
db-down:
    docker stop newsletter-postgres

# Run server in watch mode
run-watch:
    cargo watch -x run

# Run tests in watch mode
test-watch:
    cargo watch -x test

# Checks for inconsistencies, warnings and errors in the codebase
check:
    cargo fmt --all -- --check
    cargo clippy -- -D warnings
    cargo test
    cargo audit
