from options import util
import datetime
import logging
import multiprocessing
import os
import csv

root = logging.getLogger()
root.setLevel(logging.INFO)

BATCH_SIZE = 1000

def update_returns(data_path):

    # load prices and dividends into memory
    with open(f'{data_path}/eod_prices.csv', 'r') as f:
        prices = {row['symbol']: row for row in csv.DictReader(f)}

    with open(f'{data_path}/dividends.csv', 'r') as f:
        dividends = {row['dividend_symbol']: row for row in csv.DictReader(f)}

    writer = None
    with open(f'{data_path}/returns.csv', 'w') as w:
        with open(f'{data_path}/options.csv', 'r') as f:
            options_reader = csv.DictReader(f)

            for row in options_reader:
                row.update(prices[row['symbol']])
                row.update(dividends[row['symbol']])
                row['dividend_ex_date'] = datetime.datetime.strptime(row['dividend_ex_date'], '%Y-%m-%d')
                row['expiration_date'] = datetime.datetime.strptime(row['expiration_date'], '%Y%m%d')

                returns = _process(row)
                if returns is not None and returns['return_after_1_div'] is not None:
                    if writer is None:
                        writer = csv.DictWriter(w, fieldnames=returns.keys())
                        writer.writeheader()
                    writer.writerow(returns)


def _process(r):
    row = r.copy()
    row['mid'] = (float(row['bid']) + float(row['ask'])) / 2
    row['net'] = (float(row['previous_stock_price']) - float(row['mid']))
    row['premium'] = float(row['strike_price']) - float(row['net'])
    row['insurance'] = (float(row['previous_stock_price']) - float(row['net'])) / float(row['previous_stock_price'])

    # ignore unrealistic premiums
    if row['premium'] < 0.05:
        return None

    for j in range(0, 6):
        row[f'return_after_{j+1}_div'] = util.calculate_return_after_dividends(row, n_dividends=j)
    row['dividend_ex_date'] = datetime.datetime.strftime(row['dividend_ex_date'], '%Y-%m-%d')
    row['expiration_date'] = datetime.datetime.strftime(row['expiration_date'], '%Y-%m-%d')
    return row


if __name__ == '__main__':
    update_returns(data_path=os.getenv('DATA_PATH'))
