#!/bin/bash

set -ex

echo 'update prices'
time PYTHONPATH=${HOME}/src/gstvolvr/options python3 ${HOME}/src/gstvolvr/options/options/daos/daily/update_prices.py

echo 'update options'
time PYTHONPATH=${HOME}/src/gstvolvr/options python3 ${HOME}/src/gstvolvr/options/options/daos/daily/update_options.py

echo 'load to sheets'
time PYTHONPATH=${HOME}/src/gstvolvr/options python3 ${HOME}/src/gstvolvr/options/options/load_to_sheets.py
