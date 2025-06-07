#!/bin/bash

set -e

SCRIPT=`readlink -f $0`
SCRIPT_DIR=`dirname $SCRIPT`
APP_DIR=$SCRIPT_DIR/..

echo $APP_DIR
export $(cat $HOME/.dev/.env | xargs)
source $APP_DIR/.venv/bin/activate

echo "Starting Schwab API requests"
cd $APP_DIR/options-rs
time cargo run --package options-rs --bin options-rs
echo "Starting data load to Google Sheets"
time PYTHONPATH=$APP_DIR python3 $APP_DIR/options/load_to_sheets.py
echo "Starting data load to Cloud Firestore"
