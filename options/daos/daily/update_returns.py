from options import util
import datetime
import logging
import os
import csv

root = logging.getLogger()
root.setLevel(logging.INFO)

BATCH_SIZE = 1000

def update_returns(data_path):

    with open(f'{data_path}/dividends.csv', 'r') as f:
        dividends = {row['dividend_symbol']: row for row in csv.DictReader(f)}

    writer = None
    with open(f'{data_path}/returns.csv', 'w') as w:
        with open(f'{data_path}/options.csv', 'r') as f:
            options_reader = csv.DictReader(f)

            for row in options_reader:
                row.update(dividends[row['symbol']])
                row['dividend_ex_date'] = datetime.datetime.strptime(row['dividend_ex_date'], '%Y-%m-%d')
                row['expiration_date'] = datetime.datetime.fromtimestamp(int(row['expiration_date']) / 1000)

                # only look roughly 18 months out
                if row['expiration_date'] > datetime.datetime.today() + datetime.timedelta(days=30*20):
                    continue

                # we only want to consider "realistic" strike prices
                if float(row['last']) * 0.50 > float(row['strike_price']):
                    continue

                returns = _process(row)

                if returns is not None and returns['return_after_1_div'] is not None:
                    if writer is None:
                        writer = csv.DictWriter(w, fieldnames=returns.keys())
                        writer.writeheader()
                    writer.writerow(returns)


def _process(r):
    row = r.copy()
    row['mid'] = (float(row['bid']) + float(row['ask'])) / 2
    row['net'] = (float(row['last']) - float(row['mid']))
    row['premium'] = float(row['strike_price']) - float(row['net'])
    row['insurance'] = (float(row['last']) - float(row['net'])) / float(row['last'])

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
