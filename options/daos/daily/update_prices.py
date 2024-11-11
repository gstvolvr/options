from functools import partial
from typing import Optional
import csv
import logging
import multiprocessing
import os
import options.clients.polygon
import time

root = logging.getLogger()
root.setLevel(logging.INFO)

MIN_STOCK_PRICE = 7.5


def update_eod_prices(data_path):
    with open(f'{data_path}/universe.csv', 'r') as f:
        tickers = (symbol.strip() for symbol in f.readlines())

    trading_date = client.get_last_trading_date()
    func = partial(_process, date=trading_date)
    with multiprocessing.Pool(4) as p:
        list_params = p.map(func, tickers)
    if not list_params:
        return

    # remove symbols where the quote was empty
    filtered_list_params = filter(None, list_params)

    writer = None
    with open(f'{data_path}/eod_prices.csv', 'w') as f:
        for params in sorted(filtered_list_params, key=lambda x: x['ticker']):
            if writer is None:
                writer = csv.DictWriter(f, fieldnames=params.keys())
                writer.writeheader()
            if params['previous_stock_price'] and params['previous_stock_price'] > MIN_STOCK_PRICE:
                writer.writerow(params)

def _process(ticker: str, date: str) -> Optional[dict]:
    _client = options.clients.polygon.LazyClient.get_instance()
    quote = _client.get_close_price(ticker, date)
    time.sleep(0.001)
    if quote is None:
        logging.info(f'check {ticker}: quote is empty')
        return

    return {
        'ticker': ticker,
        'previous_stock_price': quote['close'],
        'previous_date': date
    }


if __name__ == '__main__':
    client = options.clients.polygon.Polygon()
    update_eod_prices(data_path=os.getenv('DATA_PATH'))
