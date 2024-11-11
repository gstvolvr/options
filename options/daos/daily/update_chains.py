from options import polygon_util
from options import util
import csv
import datetime
import logging
import os
import options.clients.polygon
import time

root = logging.getLogger()
root.setLevel(logging.INFO)


def update_eod_options(data_path):

    writer = None
    n_empty_params, n_faulty_date_params = 0, 0
    count = 0
    with open(f'{data_path}/options.csv', 'w') as w:
        with open(f'{data_path}/dividends.csv', 'r') as f:
            dividends_reader = csv.DictReader(f)

            for dividend in dividends_reader:
                count += 1
                logging.info(f'getting options chain for: {dividend["ticker"]}')
                symbol_params = _process(dividend)

                if not symbol_params:
                    n_empty_params += 1

                for date_params in symbol_params:
                    if writer is None:
                        writer = csv.DictWriter(w, fieldnames=date_params.keys())
                        writer.writeheader()
                    if date_params['close'] != 0:
                        writer.writerow(date_params)
                    else:
                        n_faulty_date_params += 1
                time.sleep(0.5)

    logging.info(f'number of empty outputs: {n_empty_params}')
    logging.info(f'number of faulty options: {n_faulty_date_params}')


def _process(item):
    _client = options.clients.polygon.LazyClient.get_instance()
    min_contract_date = item['ex_dividend_date']
    response = _client.get_chains(ticker=item['ticker'])
    data = []

    for contract in response:
        if contract['expiration_date'] < min_contract_date:
            data.append(contract)
    return data



if __name__ == '__main__':
    client = options.clients.polygon.Polygon()
    update_eod_options(data_path=os.getenv('DATA_PATH'))
