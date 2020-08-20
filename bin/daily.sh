#!/bin/bash

set -ex

SCRIPT=`readlink -f $0`
SCRIPT_DIR=`dirname $SCRIPT`
APP_DIR=$SCRIPT_DIR/..

source $APP_DIR/venv/bin/activate

echo 'update prices'
time PYTHONPATH=$APP_DIR python3 $APP_DIR/options/daos/daily/update_prices.py

echo 'update options'
time PYTHONPATH=$APP_DIR python3 $APP_DIR/options/daos/daily/update_options.py

echo 'load to sheets'
time PYTHONPATH=$APP_DIR python3 $APP_DIR/options/load_to_sheets.py
