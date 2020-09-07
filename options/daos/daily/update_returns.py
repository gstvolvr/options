from options import util
import datetime
import logging
import multiprocessing
import os
import pickle

root = logging.getLogger()
root.setLevel(logging.INFO)


def update_returns(data_path):

    with open(f'{data_path}/options.pickle', 'rb') as f:
        options = pickle.load(f)

    with open(f'{data_path}/eod_prices.pickle', 'rb') as f:
        prices = pickle.load(f)

    with open(f'{data_path}/dividends.pickle', 'rb') as f:
        dividends = pickle.load(f)

    rows = options.copy()
    for row in rows:
        row.update(prices[row['symbol']])
        row.update(dividends[row['symbol']])
        row['dividend_ex_date'] = datetime.datetime.strptime(row['dividend_ex_date'], '%Y-%m-%d')
        row['expiration_date'] = datetime.datetime.strptime(row['expiration_date'], '%Y%m%d')

    with multiprocessing.Pool(4) as p:
        returns = list(filter(None, p.map(_process, rows)))

    with open(f'{data_path}/returns.pickle', 'wb') as f:
        pickle.dump(returns, f, pickle.HIGHEST_PROTOCOL)


def _process(row):
    row['mid'] = (row['bid'] + row['ask']) / 2
    row['net'] = (row['previous_stock_price'] - row['mid'])
    row['premium'] = row['strike_price'] - row['net']
    row['insurance'] = (row['previous_stock_price'] - row['net']) / row['previous_stock_price']

    # ignore unrealistic premiums
    if row['premium'] < 0.05:
        return None

    for j in range(0, 6):
        row[f'return_after_{j+1}_div'] = util.days_to_next_event(row, i=j)
    row['dividend_ex_date'] = datetime.datetime.strftime(row['dividend_ex_date'], '%Y-%m-%d')
    row['expiration_date'] = datetime.datetime.strftime(row['expiration_date'], '%Y-%m-%d')
    return row


if __name__ == '__main__':
    update_returns(data_path=os.getenv('DATA_PATH'))
