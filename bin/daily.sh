#!/bin/bash

set -e

# Determine script directory in a Docker-friendly way
if [ -z "$APP_DIR" ]; then
  SCRIPT=$(readlink -f $0 2>/dev/null || realpath $0 2>/dev/null || echo $0)
  SCRIPT_DIR=$(dirname $SCRIPT)
  APP_DIR=$SCRIPT_DIR/..
fi

echo "Working directory: $APP_DIR"

# In Docker, environment variables are passed directly or via .env file
# The .env file is handled by the docker-entrypoint.sh script
if [ -f "/app/.env" ]; then
  export $(cat /app/.env | xargs)
elif [ -f "$HOME/.dev/.env" ]; then
  export $(cat $HOME/.dev/.env | xargs)
fi

# Only source virtual environment if not in Docker
if [ ! -f "/.dockerenv" ] && [ -f "$APP_DIR/.venv/bin/activate" ]; then
  source $APP_DIR/.venv/bin/activate
fi

#echo "Starting Schwab API requests"
#cd $APP_DIR/options-rs
#if [ -f "/.dockerenv" ]; then
#  # In Docker, use the pre-built binary
#  time $APP_DIR/options-rs/target/release/options-rs
#else
#  # Outside Docker, build and run with cargo
#  time cargo run --package options-rs --bin options-rs
#fi

echo "Starting data load to Google Sheets"
time PYTHONPATH=$APP_DIR python3 $APP_DIR/options/load_to_sheets.py

echo "Starting data load to Cloud Firestore"
time PYTHONPATH=$APP_DIR python3 $APP_DIR/options/load_to_firestore.py
