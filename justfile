# Load environment variables
set dotenv-load := true

# Launch database instance inside a Docker container
db-up:
    echo "»»» Initializing database instance..."
    sh ./scripts/init_db.sh

# Run migrations against the current PostgreSQL instance
db-migrate $SKIP_DOCKER="true":
    sh ./scripts/init_db.sh

# Stop running database instance and remove its container
db-down:
    echo "»»» Stopping database instance and cleaning up container..."
    docker stop newsletter-postgres

# Run server in watch mode
run-watch:
    cargo watch -x run

# Run tests in watch mode
test-watch:
    cargo watch -x test

# Checks for inconsistencies, warnings and errors in the codebase
inspect:
    echo "Running local source code inspections..."
    echo "Formatting issues (rustfmt)"
    cargo fmt --all -- --check
    echo "Linting violations (clippy)"
    cargo clippy -- -D warnings
    echo "Failing tests (cargo test)"
    cargo test
    echo "Dependency vulnerabilities in Cargo.lock file (cargo-audit)"
    cargo audit
