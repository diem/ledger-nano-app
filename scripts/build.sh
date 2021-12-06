#!/bin/bash

# Build rust-app
docker run --rm -it \
  -v $(pwd)/rust-app:/app \
  -v $(pwd)/ledger-nanos-sdk:/ledger-nanos-sdk  \
  -v $(pwd)/ledger-nanos-ui:/ledger-nanos-ui  \
  --entrypoint bash ledger_build -c 'cd /app && . "$HOME/.cargo/env" && cargo build --release && cargo ledger'

# # Build ledger-nanos-sdk
# docker run --rm -it \
#   -v $(pwd)/rust-app:/app \
#   -v $(pwd)/ledger-nanos-sdk:/ledger-nanos-sdk  \
#   -v $(pwd)/ledger-nanos-ui:/ledger-nanos-ui  \
#   --entrypoint bash ledger_build -c 'cd /ledger-nanos-sdk && . "$HOME/.cargo/env" && cargo build --release'
