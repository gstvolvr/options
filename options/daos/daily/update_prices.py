import logging
import psycopg2
import options.iex
import datetime
import os
import multiprocessing
from dateutil.relativedelta import relativedelta

root = logging.getLogger()
root.setLevel(logging.INFO)


def update_eod_prices(conn, iex):

    sql = """
    SELECT symbol
    FROM universe.symbols
    WHERE in_universe = TRUE
    ORDER BY symbol;
    """

    update_sql = """
    INSERT INTO universe.eod_prices(
        symbol, latest_stock_price, latest_date, previous_stock_price, previous_date, db_updated
    ) VALUES (
        %(symbol)s,
        %(latest_stock_price)s,
        %(latest_date)s,
        %(previous_stock_price)s,
        %(previous_date)s,
        NOW()
    )
    ON CONFLICT(symbol)
    DO UPDATE SET
        symbol = EXCLUDED.symbol,
        latest_stock_price = EXCLUDED.latest_stock_price,
        latest_date = EXCLUDED.latest_date,
        previous_stock_price = EXCLUDED.previous_stock_price,
        previous_date = EXCLUDED.previous_date,
        db_updated = NOW()
    """

    with conn.cursor() as update_cursor:
        with conn.cursor() as cursor:
            cursor.execute('TRUNCATE universe.eod_prices;')
            cursor.execute(sql)

            symbols = [symbol for (symbol,) in cursor.fetchall()]

            with multiprocessing.Pool(4) as p:
                params = p.map(_process, symbols)
            update_cursor.executemany(update_sql, list(filter(None, params)))


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
        previous_date = date - datetime.timedelta(days=1)
        return {
            'symbol': symbol,
            'latest_stock_price': latest_price,
            'latest_date': date.strftime('%Y-%m-%d'),
            'previous_stock_price': quote['previousClose'],
            'previous_date': previous_date.strftime('%Y-%m-%d')}

    elif quote['closeSource'] == 'official':
        latest_price = quote['close']
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

    with psycopg2.connect(dbname=os.getenv('DB_NAME'),
                          user=os.getenv('DB_USER'),
                          password=os.getenv('DB_PASS'),
                          host=os.getenv('DB_HOST'),
                          port=os.getenv('DB_PORT')) as conn:
        update_eod_prices(conn, iex)
