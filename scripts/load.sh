#!/bin/bash

# Install on nano device
cd rust-app
pipenv run python ../ledgerctl/ledgerctl.py install -f app.json
