from dateutil.relativedelta import relativedelta
import datetime
import logging
import multiprocessing
import options.iex
import options.util
import os
import csv
from functools import partial
import time

root = logging.getLogger()
root.setLevel(logging.INFO)

MIN_STOCK_PRICE = 7.5


def update_eod_prices(data_path):
    with open(f'{data_path}/universe.csv', 'r') as f:
        symbols = [symbol.strip() for symbol in f.readlines()]

    today = datetime.datetime.today()
    weekend = today.weekday() in [5, 6]
    # TODO: handle offset on holidays
    offset = 2 if weekend else 1
    trading_date = iex.get_last_trading_day(offset=offset)
    print(trading_date)

    func = partial(_process, date=trading_date)
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

def _process(symbol, date):
  quote = iex.get_quote_from_date(symbol, date)
  time.sleep(0.001)
  if quote is None:
      logging.info(f'check {symbol}: quote is empty')
      return
  return {
      'symbol': symbol,
      'previous_stock_price': quote,
      'previous_date': date}


if __name__ == '__main__':
    iex = options.iex.IEX()
    iex.token = os.getenv('IEX_TOKEN')
    update_eod_prices(data_path=os.getenv('DATA_PATH'))
