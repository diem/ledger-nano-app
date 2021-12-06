FROM ubuntu

RUN apt update

RUN apt install -y curl git gcc-arm-none-eabi binutils-arm-none-eabi gcc-multilib python3 python3-pip clang

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

RUN . "$HOME/.cargo/env" && rustup default nightly && rustup target add thumbv6m-none-eabi

RUN . "$HOME/.cargo/env" && cargo install --git https://github.com/LedgerHQ/cargo-ledger && mkdir /app && cd /app && git clone https://github.com/LedgerHQ/rust-app && cd rust-app && cargo build
