#!/usr/bin/env bash

# Script for initializing the database using Docker and the SQLx CLI

set -x
set -eo pipefail

# Check that required executables are installed (psql, sqlx)
if ! [ -x "$(command -v psql)" ]; then
  echo >&2 "[-] Error: 'psql' must be installed."
  exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 "[-] Error: 'sqlx-cli' must be installed."
  echo >&2 "    To install the SQLx CLI using cargo, run:"
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
    -d postgres \
    postgres -N ${DB_MAX_CONNECTIONS}
fi

export PGPASSWORD="${DB_PASSWORD}"
# Ping PostgreSQL service until it is ready to accept commands
until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "${DB_NAME}" -c '\q'; do
  echo >&2 "[-] PostgreSQL service unavailable. Retrying..."
  sleep 1
done

echo >&2 "[*] PostgreSQL service running on port ${DB_PORT}. Executing migrations..."

# 'DATABASE_URL' must be set for SQLx to work properly, so we explicitly set it
export DATABASE_URL="postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}"
# Create database and then run existing migrations with SQLx CLI
sqlx database create
sqlx migrate run

echo >&2 "[*] PostgreSQL database migrations successful."
