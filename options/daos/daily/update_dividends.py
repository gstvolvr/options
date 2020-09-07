from dateutil.relativedelta import relativedelta
from options import util
import datetime
import logging
import options.iex
import os
import pickle
import re

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

    with open(f'{data_path}/eod_prices.pickle', 'rb') as f:
        prices = pickle.load(f)
        symbols = prices.keys()

    dividends = {}
    today = datetime.datetime.strftime(datetime.date.today(), '%Y-%m-%d')
    for i, symbol in enumerate(symbols):
        if (i != 0) and (i % 100) == 0:
            logging.info(f'processed: {i}')

        dividend = iex.get_next_dividend(symbol)

        if dividend == {}:
            dividend['calculated'] = True
            dividend = iex.get_last_dividend(symbol)
            if 'frequency' not in dividend or dividend['frequency'] not in util.FREQUENCY_MAPPING:
                continue

            dividend['exDate'] = _add_months(dividend['exDate'], util.FREQUENCY_MAPPING[dividend['frequency']])
        else:
            dividend['calculated'] = False

        if 'frequency' not in dividend or dividend['frequency'] not in util.FREQUENCY_MAPPING:
            continue

        dividend_clean = {'dividend_' + to_snake(k): _clean(v) for k, v in dividend.items()}

        # ignore non-cash dividends
        if dividend_clean['dividend_flag'] != 'Cash' or \
                dividend_clean['dividend_amount'] is None or \
                dividend_clean['dividend_ex_date'] < today:
                    continue
        # once we know `amount` is not None
        dividend_clean['gross_annual_yield'] = float(dividend_clean['dividend_amount']) * \
                                               (12. / util.FREQUENCY_MAPPING[dividend_clean['dividend_frequency']])

        if dividend_clean['gross_annual_yield'] / prices[symbol]['previous_stock_price'] <= .02:
            continue

        dividends[symbol] = dividend_clean

    with open(f'{data_path}/dividends.pickle', 'wb') as f:
        pickle.dump(dividends, f, pickle.HIGHEST_PROTOCOL)


if __name__ == '__main__':
    iex = options.iex.IEX()
    iex.token = os.getenv('IEX_TOKEN')

    update_dividends(data_path=os.getenv('DATA_PATH'))
