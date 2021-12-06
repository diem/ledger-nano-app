#!/bin/bash

docker run --rm -it -v $(pwd)/rust-app/target/thumbv6m-none-eabi/release:/speculos/apps -p 1234:1234 -p 5001:5000 -p 40000:40000 -p 41000:41000 ghcr.io/ledgerhq/speculos --model nanos --sdk 2.0 --seed "secret" --display headless --apdu-port 40000 --vnc-port 41000 ./apps/trove-wallet
