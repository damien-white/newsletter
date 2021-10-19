#!/usr/bin/env bash

set -eox pipefail

# Verify that the system has the required bins installed before proceeding
function check_deps() {
  local required_deps=(psql sqlx)
  for item in "${required_deps[@]}"; do
    echo "${item}"
    if ! [ -x "$(command -v ${item})" ]; then
      echo >&2 "[ERR]: ${item} is either missing or lacks executable permissions."
      exit 1
    fi
  done
}

check_deps

# Create a new environment variable file and use direnv to set values
function load_from_env() {
  local envfile="./.env"
  set -a && . $envfile
}

load_from_env

# Set environment variables to be used to initialize database
POSTGRES_USER="${POSTGRES_USER:=postgres}"
POSTGRES_PASSWORD="${POSTGRES_PASSWORD:=password}"
POSTGRES_HOST="${POSTGRES_HOST:=localhost}"
POSTGRES_PORT="${POSTGRES_PORT:=5432}"
POSTGRES_DB="${POSTGRES_DB:=newsletter}"
POSTGRES_CONCURRENCY="${POSTGRES_CONCURRENCY:=1000}"

# Start PostgreSQL database with Docker unless 'SKIP_DOCKER' flag is set
if [[ -z "${SKIP_DOCKER}" ]]; then
  docker run --rm \
    --name newsletter-postgres \
    -e POSTGRES_USER=${POSTGRES_USER} \
    -e POSTGRES_PASSWORD=${POSTGRES_PASSWORD} \
    -e POSTGRES_DB=${POSTGRES_DB} \
    -p "${POSTGRES_PORT}:5432" \
    -d postgres:14.0 \
    postgres -N ${POSTGRES_CONCURRENCY}
fi

# 'DATABASE_URL' must be set for SQLx to work properly, so we explicitly set it
export DATABASE_URL="postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${POSTGRES_HOST}:${POSTGRES_PORT}/${POSTGRES_DB}"

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
    echo >&2 "[WARN] Cannot reach PostgreSQL service. Retrying in ${delay} seconds..."
    sleep $delay
  done
}

# Run `wait_for_postgres` function
wait_for_postgres

echo >&2 "PostgreSQL service started successfully. Executing migrations..."
# Create database and run any pending migrations with `sqlx-cli`
sqlx database create
sqlx migrate run

echo >&2 "Database migrations complete."
