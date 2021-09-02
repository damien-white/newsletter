#!/usr/bin/env bash

set -x
set -eo pipefail

if ! [ -x "$(command -v psql)" ]; then
  echo >&2 "[-] Error: 'psql' must be installed."
  exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 "[-] Error: 'sqlx-cli' must be installed. To install with cargo, use:"
  echo >&2 "    cargo install sqlx-cli --no-default-features --features postgres"
  exit 1
fi

# Set environment variables to be used to initialize database
DB_USER="${POSTGRES_USER:=postgres}"
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_PORT="${POSTGRES_PORT:=5432}"
DB_NAME="${POSTGRES_DB:=newsletter}"
DB_MAX_CONNECTIONS="${POSTGRES_CONCURRENCY:=1000}"

# Start PostgreSQL database with Docker unless 'SKIP_DOCKER' flag is set
if [[ -z "${SKIP_DOCKER}" ]]; then
  docker run --rm \
    --name newsletter-postgres \
    -e POSTGRES_USER=${DB_USER} \
    -e POSTGRES_PASSWORD=${DB_PASSWORD} \
    -e POSTGRES_DB=${DB_NAME} \
    -p "${DB_PORT}:5432" \
    -d postgres:13.4 \
    postgres -N ${DB_MAX_CONNECTIONS}
fi

# 'DATABASE_URL' must be set for SQLx to work properly, so we explicitly set it
export DATABASE_URL="postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}"

# Helper function that waits until PostgreSQL is up and ready to accept commands
function wait_for_postgres() {
  # Number of connection attempts made
  local attempt_count=1
  # Delay between connection attempts to implement a simple backoff strategy
  local delay=1

  # Ping PostgreSQL service until it is ready to accept commands
  until psql "${DATABASE_URL}" --command '\q'; do
    ((attempt_count = attempt_count + 1))
    ((delay = delay + 1))
    echo >&2 "[»] Cannot reach PostgreSQL service. Retrying in ${delay} seconds..."
    sleep $delay
  done
}

# Run `wait_for_postgres` function
wait_for_postgres

echo >&2 "[*] PostgreSQL service started successfully. Executing migrations..."
# Create database and run any pending migrations with `sqlx-cli`
sqlx database create
sqlx migrate run

echo >&2 "[*] Database migrations complete."
