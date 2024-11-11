from dateutil.relativedelta import relativedelta
from options import util
import polygon
import datetime
import logging
import options.iex
import os
import csv

root = logging.getLogger()
root.setLevel(logging.INFO)

__TEST_TICKER = 'BEN'

def update_eod_options(data_path):

    # sanity check – avoid running full update if numbers are not up to date
    today = datetime.datetime.today()
    weekend = today.weekday() in [5, 6]
    delta = 3 if today.weekday() == 0 else 1
    yesterday = today - relativedelta(days=delta)
    yesterday_fmt = datetime.datetime.strftime(yesterday, '%Y-%m-%d')
    dates = iex.get_call_expiration_dates(__TEST_TICKER)
    results = iex.get_calls(__TEST_TICKER, dates[0])

    # TODO: actually parse dates
    if results[0]['lastUpdated'].replace('-', '') != yesterday_fmt.replace('-', '') and not weekend:
        logging.info(results[0])
        raise Exception(f"Numbers haven't been updated to {yesterday_fmt}")

    writer = None
    n_empty_params, n_faulty_date_params = 0, 0
    with open(f'{data_path}/options.csv', 'w') as w:
        with open(f'{data_path}/dividends.csv', 'r') as f:
            dividends_reader = csv.DictReader(f)

            for dividend in dividends_reader:
                symbol_params = _process(dividend)

                if not symbol_params:
                    n_empty_params += 1

                for date_params in symbol_params:
                    if writer is None:
                        writer = csv.DictWriter(w, fieldnames=date_params.keys())
                        writer.writeheader()
                    if not date_params['is_adjusted'] and date_params['ask'] != 0:
                        writer.writerow(date_params)
                    else:
                        n_faulty_date_params += 1

    logging.info(f'number of empty outputs: {n_empty_params}')
    logging.info(f'number of faulty options: {n_faulty_date_params}')


def _process(item):
    min_contract_date = item['dividend_ex_date'][:5].replace('-', '')
    dates = iex.get_call_expiration_dates(item['dividend_symbol'])
    params = []

    for expiration_date in dates:
        if expiration_date >= min_contract_date:
            results = iex.get_calls(item['dividend_symbol'], expiration_date)
            if results:
                params.extend([{util.to_snake(k): v for k, v in instance.items()} for instance in results])
    return params


if __name__ == '__main__':
    client = polygon.RESTClient()
    update_eod_options(data_path=os.getenv('DATA_PATH'))
