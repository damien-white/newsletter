#!/usr/bin/env bash

set -x
set -eo pipefail

# Set environment variables to be used to initialize database
DB_USER="${POSTGRES_USER:=postgres}"
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB:=newsletter}"
DB_PORT="${POSTGRES_PORT:=5432}"
DB_MAX_CONNECTIONS="${POSTGRES_CONCURRENCY:=1000}"

# Start PostgreSQL database with Docker
docker run --rm \
  --name newsletter-postgres \
  -e POSTGRES_USER=${DB_USER} \
  -e POSTGRES_PASSWORD=${DB_PASSWORD} \
  -e POSTGRES_DB=${DB_NAME} \
  -p "${DB_PORT}:5432" \
  -d postgres \
  postgres -N ${DB_MAX_CONNECTIONS}

export PGPASSWORD="${DB_PASSWORD}"
# Ping PostgreSQL service until it is ready to accept commands
until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
  >&2 echo "PostgreSQL service unavailable. Retrying..."
  sleep 1
done

>&2 echo "»»» PostgreSQL service is running on port ${DB_PORT}"

sqlx database create
