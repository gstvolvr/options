from dateutil.relativedelta import relativedelta
from options import util, polygon_util
import datetime
import logging
import options.clients.polygon
import os
import re
import csv

root = logging.getLogger()
root.setLevel(logging.INFO)


def to_snake(name):
    return re.sub(f'(?<!^)(?=[A-Z])', '_', name).lower()


def update_dividends(data_path):
    def _clean(value):
        if value == '':
            return None
        return value

    def _add_months(date, months=3):
        new_date = datetime.datetime.strptime(date, '%Y-%m-%d') + relativedelta(months=months)
        return datetime.datetime.strftime(new_date, '%Y-%m-%d')

    today = datetime.datetime.strftime(datetime.date.today(), '%Y-%m-%d')
    writer = None
    n_ignored_dividends = 0
    with open(f'{data_path}/dividends.csv', 'w') as w:
        with open(f'{data_path}/eod_prices.csv', 'r') as r:
            prices = csv.DictReader(r)

            for i, row in enumerate(prices):
                if (i != 0) and (i % 100) == 0:
                    logging.info(f'processed: {i}')

                ticker = row['ticker']
                logging.debug(f'Updating dividends for: {ticker}')
                dividend = client.get_next_dividend(ticker)

                if dividend is None:
                    dividend = client.get_last_dividend(ticker)
                    dividend['calculated'] = True
                    dividend['ex_dividend_date'] = _add_months(dividend['ex_dividend_date'], dividend['frequency'])
                else:
                    dividend['calculated'] = False

                if not dividend['frequency']:
                    n_ignored_dividends += 1
                    continue

                # ignore non-cash dividends
                if dividend['cash_amount'] is None or \
                   dividend['ex_dividend_date'] < today:
                        n_ignored_dividends += 1
                        continue
                # once we know `amount` is not None
                dividend['gross_annual_yield'] = float(dividend['cash_amount']) * (12. / dividend['frequency'])
                if dividend['gross_annual_yield'] / float(row['previous_stock_price']) <= .005:
                    n_ignored_dividends += 1
                    continue

                if writer is None:
                    writer = csv.DictWriter(w, fieldnames=dividend.keys())
                    writer.writeheader()
                writer.writerow(dividend)

    logging.info(f'Ignoring {n_ignored_dividends} dividends')


if __name__ == '__main__':
    client = options.clients.polygon.Polygon()
    update_dividends(data_path=os.getenv('DATA_PATH'))
