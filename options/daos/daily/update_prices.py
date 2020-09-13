from dateutil.relativedelta import relativedelta
import datetime
import logging
import multiprocessing
import options.iex
import options.util
import os
import csv

root = logging.getLogger()
root.setLevel(logging.INFO)

MIN_STOCK_PRICE = 7.5


def update_eod_prices(data_path):
    with open(f'{data_path}/symbols.csv', 'r') as f:
        symbols = [symbol.strip() for symbol in f.readlines()]

    with multiprocessing.Pool(4) as p:
        list_params = p.map(_process, symbols)

    if not list_params:
        return

    writer = None
    with open(f'{data_path}/eod_prices.csv', 'w') as f:
        for params in sorted(list_params, key=lambda x: x['symbol']):
            if writer is None:
                writer = csv.DictWriter(f, fieldnames=params.keys())
                writer.writeheader()
            if params['previous_stock_price'] > MIN_STOCK_PRICE:
                writer.writerow(params)


def _process(symbol):
    quote = iex.get_quote(symbol)
    expected_date = options.util.get_previous_trading_date()

    if quote is None:
        logging.info(f'check {symbol}: quote is empty')
        return

    close_date = datetime.datetime.fromtimestamp(quote['closeTime'] / 1000) - relativedelta(days=1)
    close_date = close_date.strftime('%Y-%m-%d')

    # when we're looking for prices two business days ago
    if quote['closeSource'] == 'official' and close_date == expected_date:
        return {
            'symbol': symbol,
            'previous_stock_price': quote['previousClose'],
            'previous_date': close_date}

    # looking for last business days price
    elif quote['latestSource'] == 'Close':
        if quote['closeTime'] is None:
            price = quote['latestPrice']
            date = datetime.datetime.strptime(quote['latestTime'], '%B %d, %Y')
            logging.info(f'{symbol}: using latest source')
        else:
            price = quote['close']
            date = datetime.datetime.fromtimestamp(quote['closeTime'] / 1000)
        return {
            'symbol': symbol,
            'previous_stock_price': price,
            'previous_date': date.strftime('%Y-%m-%d')}



if __name__ == '__main__':
    iex = options.iex.IEX()
    iex.token = os.getenv('IEX_TOKEN')
    update_eod_prices(data_path=os.getenv('DATA_PATH'))
