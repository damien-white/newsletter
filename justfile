# Load environment variables
set dotenv-load := true

# Launch database service inside Docker container
db-init:
    sh ./scripts/init_db.sh

# Run migrations against the current PostgreSQL service
db-migrate $SKIP_DOCKER="true":
    sh ./scripts/init_db.sh

# Stop database service and remove Docker container
db-stop:
    docker stop newsletter-postgres

# Stop database service, remove the container and restart the service
db-refresh:
    docker stop newsletter-postgres
    sh ./scripts/init_db.sh

# Launch server service inside Docker container
server-start:
    docker run --rm --name newsletter-server -p "8120:8120" -d newsletter

# Stop server service and remove Docker container
server-stop:
    docker stop newsletter-server

# Checks for inconsistencies, warnings and errors in the codebase
check:
    cargo fmt --all -- --check
    cargo clippy -- -D warnings
    cargo test
    cargo audit --ignore RUSTSEC-2020-0071 --ignore RUSTSEC-2020-0159
