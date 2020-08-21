#!/bin/bash

set -ex

SCRIPT=`readlink -f $0`
SCRIPT_DIR=`dirname $SCRIPT`
APP_DIR=$SCRIPT_DIR/..

export $(cat $HOME/.dev/.env | xargs)
source $APP_DIR/venv/bin/activate

time PYTHONPATH=$APP_DIR python3 $APP_DIR/options/daos/daily/update_prices.py 
sleep 5
time PYTHONPATH=$APP_DIR python3 $APP_DIR/options/daos/daily/update_dividends.py 
sleep 5
time PYTHONPATH=$APP_DIR python3 $APP_DIR/options/daos/daily/update_options.py 
sleep 5
time PYTHONPATH=$APP_DIR python3 $APP_DIR/options/daos/daily/update_returns.py 
sleep 5
time PYTHONPATH=$APP_DIR python3 $APP_DIR/options/load_to_sheets.py
