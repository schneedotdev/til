alias c := create
alias f := format

# create a til entry
create:
    cargo run -q -- that -m "new bullet point" -t "example"

# formatting - can pass an optional `--check` flag to run format check
format *FLAGS:
    cargo fmt --all -- {{ FLAGS }}
    just --fmt --unstable {{ FLAGS }}

# linting
lint:
    cargo clippy --all-targets --all-features

# run checks before pull request
pre-publish:
    RUSTFLAGS="-D warnings" just lint
    just format --check
    cargo diet -r --dry-run
    cargo publish --dry-run
