set dotenv-load

# ex: just runner-dev
runner-dev *ARGS:
    cargo run --bin hypha-runner {{ ARGS }}

runner-check *ARGS:
    cargo check --message-format=short --bin hypha-runner {{ ARGS }}
