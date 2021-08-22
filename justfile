# Load environment variables
set dotenv-load := true

# Run server in watch mode
run-watch:
    cargo watch -x run

# Run tests in watch mode
test-watch:
    cargo watch -x test

# Checks for inconsistencies, warnings and errors in the codebase
inspect:
    @echo "Running local source code inspections..."
    @echo "Formatting issues (rustfmt)"
    cargo fmt --all -- --check
    @echo "Linting violations (clippy)"
    cargo clippy -- -D warnings
    @echo "Failing tests (cargo test)"
    cargo test
    @echo "Dependency vulnerabilities in Cargo.lock file (cargo-audit)"
    cargo audit
