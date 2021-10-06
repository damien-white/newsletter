# Load environment variables
set dotenv-load := true

# Launch database instance inside Docker container
db-start:
    sh ./scripts/init_db.sh

# Run migrations against the current PostgreSQL instance
db-migrate $SKIP_DOCKER="true":
    sh ./scripts/init_db.sh

# Stop database instance and remove Docker container
db-stop:
    docker stop newsletter-postgres

# Launch server instance inside Docker container
server-start:
    docker run --rm --name newsletter-server -p "8120:8120" -d newsletter

# Stop server instance and remove Docker container
server-stop:
    docker stop newsletter-server

# Checks for inconsistencies, warnings and errors in the codebase
check:
    cargo fmt --all -- --check
    cargo clippy -- -D warnings
    cargo test
    cargo audit
