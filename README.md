# Ledger Nano S App for Signing Diem Transactions

This repo contains an experimental and Work In Progress app for Ledger Nano S for signing Diem Trasactions.

## Development Guide

The following instructions are for development on a Mac

For development, please install
* docker : https://docs.docker.com/desktop/mac/install/
* python 3 : https://docs.python-guide.org/starting/install3/osx/
* pipenv : https://docs.python-guide.org/dev/virtualenvs/#virtualenvironments-ref
* rust toolchain : https://www.rust-lang.org/tools/install

## Build the docker image for building the ledger rust app

This creates a docker image which can be used for building and testing the ledger rust application against the right compilation target.

```
cd docker_image

docker build . --tag ledger_build
```

## Build the app

This uses the `ledger_build` docker image built previously. This creates a temporary container, mounts the correct directories inside it and runs the compilation commands. After the app is built, it can be loaded on either a simulator(speculos) or the actual device (nano s) for testing.

```
./scripts/build.sh
```

## Load the app on a speculos (nano s simulator)

```
./scripts/speculos-load.sh
```

Visit localhost:5001 to interact with the simulated device and the app

## Load the app on a nano s

This needs the physical device to be connected to your computer

```
./scripts/load.sh
```

## WIP Section

## Run commands

pipenv shell

cd rust-app/test

python test_cmds.py
