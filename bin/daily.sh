#!/bin/bash

set -e

# This script is designed to run inside Docker containers
# Working directory is always /app in Docker
APP_DIR="/app"

echo "Working directory: $APP_DIR"

# Load environment variables from .env file
if [ -f "/app/.env" ]; then
  export $(cat /app/.env | xargs)
fi

# Set Google Cloud credentials path for Docker
export GOOGLE_APPLICATION_CREDENTIALS="/app/secrets-manager-key.json"

echo "Starting Schwab API requests"
cd $APP_DIR/options-rs
time RUST_LOG=info $APP_DIR/options-rs/target/release/options-rs

echo "Starting data load to Google Sheets"
time PYTHONPATH=$APP_DIR python3 $APP_DIR/options/load_to_sheets.py

echo "Starting data load to Cloud Firestore"
#time PYTHONPATH=$APP_DIR python3 $APP_DIR/options/load_to_firestore.py
