set dotenv-load

check-runner *ARGS:
    cargo check --message-format=short --bin hypha-runner {{ ARGS }}
