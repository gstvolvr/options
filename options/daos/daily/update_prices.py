from functools import partial
import csv
import logging
import multiprocessing
import options.td_ameritrade
import options.util
import os
import time

root = logging.getLogger()
root.setLevel(logging.INFO)

MIN_STOCK_PRICE = 7.5


def update_eod_prices(data_path):
    with open(f'{data_path}/universe.csv', 'r') as f:
        symbols = [symbol.strip() for symbol in f.readlines()]

    func = partial(_process)
    with multiprocessing.Pool(4) as p:
        list_params = p.map(func, symbols)
    if not list_params:
        return

    # remove symbols where the quote was empty
    filtered_list_params = filter(None, list_params)

    writer = None
    with open(f'{data_path}/eod_prices.csv', 'w') as f:
        for params in sorted(filtered_list_params, key=lambda x: x['symbol']):
            if writer is None:
                writer = csv.DictWriter(f, fieldnames=params.keys())
                writer.writeheader()
            if params['previous_stock_price'] and params['previous_stock_price'] > MIN_STOCK_PRICE:
                writer.writerow(params)


def _process(symbol):
    quote = client.get_quote(symbol)
    time.sleep(0.001)
    if not quote:
        logging.info(f'check {symbol}: quote is empty')
        return
    return {
        'symbol': symbol,
        'previous_stock_price': quote[symbol]['lastPrice']
    }


if __name__ == '__main__':
    client = options.td_ameritrade.TDAmeritrade()
    client.token = os.getenv('TD_AMERITRADE_TOKEN')
    update_eod_prices(data_path=os.getenv('DATA_PATH'))
