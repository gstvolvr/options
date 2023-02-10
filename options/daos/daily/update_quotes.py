from dateutil.relativedelta import relativedelta
import csv
import datetime
import json
import logging
import options.td_ameritrade
import options.util
import os
import time

root = logging.getLogger()
root.setLevel(logging.INFO)

MIN_STOCK_PRICE = 7.5


def _add_months(date: datetime.date, months=3):
    new_date = date + relativedelta(months=months)
    return datetime.datetime.strftime(new_date, '%Y-%m-%d')


def update_uptes(data_path):
    """
    TD Ameritrade has a restrictive API limit. Can't parallelize.
    """
    with open(f'{data_path}/universe.csv', 'r') as f:
        quotes = {}
        today = datetime.date.today()
        for symbol in f.readlines():
            symbol = symbol.strip()
            quote = _process(symbol)

            if quote and quote['previous_stock_price'] and \
                    quote['previous_stock_price'] > MIN_STOCK_PRICE and \
                    quote['dividend_annual_yield'] > .005:

                div_date = datetime.datetime.fromisoformat(quote['dividend_ex_date']).date()

                # they probably haven't announced the next dividend, we should make an educated guess
                if div_date < today:
                    quote['dividend_calculated'] = True
                    quote['dividend_ex_date'] = _add_months(div_date)
                else:
                    quote['dividend_calculated'] = False
                    quote['dividend_ex_date'] = datetime.datetime.strftime(div_date, '%Y-%m-%d')

                quotes[symbol] = quote
    if not quotes:
        return

    with open(f'{data_path}/quotes.json', 'w') as f:
        json.dump(quotes, f, indent=4, sort_keys=True)


def _process(symbol):
    quote = client.get_quote(symbol)
    if 'error' in quote:
        raise Exception(quote['error'])
    if not quote or symbol not in quote:
        logging.info(f'check {symbol}: quote is empty')
        return
    return {
        'previous_stock_price': quote[symbol]['lastPrice'],
        'dividend_annual_amount': quote[symbol]['divAmount'],
        'dividend_annual_yield': quote[symbol]['divYield'],
        'dividend_quarterly_amount': quote[symbol]['divAmount'] / 4,
        'dividend_ex_date': quote[symbol]['divDate'],
        'asset_type': quote[symbol]['assetType'],
    }


if __name__ == '__main__':
    client = options.td_ameritrade.TDAmeritrade()
    client.token = os.getenv('TD_AMERITRADE_TOKEN')
    update_uptes(data_path=os.getenv('DATA_PATH'))
