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
# In Cloud Run, use the service account; locally use the key file
if [ ! -f "/app/secrets-manager-key.json" ]; then
  echo "Using Cloud Run service account for authentication"
  unset GOOGLE_APPLICATION_CREDENTIALS
else
  export GOOGLE_APPLICATION_CREDENTIALS="/app/secrets-manager-key.json"
fi

echo "Starting Schwab API requests"
cd $APP_DIR/options-rs
time RUST_LOG=info $APP_DIR/options-rs/target/release/options-rs

echo "Starting data load to Google Sheets"
time PYTHONPATH=$APP_DIR python3 $APP_DIR/options/load_to_sheets.py

#echo "Starting data load to Cloud Firestore"
#time PYTHONPATH=$APP_DIR python3 $APP_DIR/options/load_to_firestore.py
