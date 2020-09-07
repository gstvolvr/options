from dateutil.relativedelta import relativedelta
from options import util
import datetime
import logging
import multiprocessing
import options.iex
import os
import pickle

root = logging.getLogger()
root.setLevel(logging.INFO)


def update_eod_options(data_path):

    with open(f'{data_path}/dividends.pickle', 'rb') as f:
        dividends = pickle.load(f)
    with open(f'{data_path}/eod_prices.pickle', 'rb') as f:
        prices = pickle.load(f)

    keys = dividends.keys()
    for d in keys:
        dividends[d].update(prices[d])

    # sanity check – avoid running full update if numbers are not up to date
    today = datetime.datetime.today()
    weekend = today.weekday() in [5, 6]
    delta = 3 if today.weekday() == 0 else 1
    yesterday = today - relativedelta(days=delta)
    yesterday_fmt = datetime.datetime.strftime(yesterday, '%Y-%m-%d')
    dates = iex.get_call_expiration_dates('BEN')
    results = iex.get_calls('BEN', dates[0])

    if results[0]['lastUpdated'] != yesterday_fmt and not weekend:
        logging.info(results[0])
        raise Exception(f"Numbers haven't been updated to {yesterday_fmt}")

    with multiprocessing.Pool(2) as p:
        params = p.map(_process, dividends.items())

    options = [p for symbol_params in params for p in symbol_params]

    with open(f'{data_path}/options.pickle', 'wb') as f:
        pickle.dump(options, f, pickle.HIGHEST_PROTOCOL)


def _process(items):
    symbol, values = items
    min_contract_date = values['dividend_ex_date'][:5].replace('-', '')

    dates = iex.get_call_expiration_dates(symbol)
    params = []

    for expiration_date in dates:
        if expiration_date >= min_contract_date:
            results = iex.get_calls(symbol, expiration_date)
            if results:
                params.extend([{util.to_snake(k): v for k, v in instance.items()} for instance in results])
    return params


if __name__ == '__main__':
    iex = options.iex.IEX()
    iex.token = os.getenv('IEX_TOKEN')
    update_eod_options(data_path=os.getenv('DATA_PATH'))
