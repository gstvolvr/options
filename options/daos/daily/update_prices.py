from dateutil.relativedelta import relativedelta
import datetime
import logging
import multiprocessing
import options.iex
import os
import pickle

root = logging.getLogger()
root.setLevel(logging.INFO)

MIN_STOCK_PRICE = 7.5


def update_eod_prices(data_path):
    with open(f'{data_path}/symbols.csv', 'r') as f:
        symbols = [symbol.strip() for symbol in f.readlines()]

    with multiprocessing.Pool(4) as p:
        list_params = p.map(_process, symbols[:10])

    prices = {}
    for params in list_params:
        if params['previous_stock_price'] > MIN_STOCK_PRICE:
            symbol = params.pop('symbol')
            prices[symbol] = params

    with open(f'{data_path}/eod_prices.pickle', 'wb') as f:
        pickle.dump(prices, f, pickle.HIGHEST_PROTOCOL)


def _process(symbol):
    quote = iex.get_quote(symbol)
    if quote is None:
        logging.info(f'check {symbol}: quote is empty')
        return
    if quote['latestSource'] == 'Close':
        if quote['closeTime'] is None:
            latest_price = quote['latestPrice']
            date = datetime.datetime.strptime(quote['latestTime'], '%B %d, %Y')
            logging.info(f'{symbol}: using latest source')
        else:
            latest_price = quote['close']
            date = datetime.datetime.fromtimestamp(quote['closeTime'] / 1000)
        return {
            'symbol': symbol,
            'latest_stock_price': None,
            'latest_date': None,
            'previous_stock_price': latest_price,
            'previous_date': date.strftime('%Y-%m-%d')}

    elif quote['closeSource'] == 'official':
        yesterday = datetime.datetime.today() - relativedelta(days=1)
        return {
            'symbol': symbol,
            'latest_stock_price': None,
            'latest_date': None,
            'previous_stock_price': quote['previousClose'],
            'previous_date': yesterday.strftime('%Y-%m-%d')}
        


if __name__ == '__main__':
    iex = options.iex.IEX()
    iex.token = os.getenv('IEX_TOKEN')
    update_eod_prices(data_path=os.getenv('DATA_PATH'))
