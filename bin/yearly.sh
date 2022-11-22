#!/bin/bash

set -e

SCRIPT=`readlink -f $0`
SCRIPT_DIR=`dirname $SCRIPT`
APP_DIR=$SCRIPT_DIR/..

export $(cat $HOME/.dev/.env | xargs)
source $APP_DIR/.venv/bin/activate

time PYTHONPATH=$APP_DIR python3 $APP_DIR/options/daos/yearly/update_universe.py
time PYTHONPATH=$APP_DIR python3 $APP_DIR/options/daos/yearly/update_companies.py
