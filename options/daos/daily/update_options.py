import datetime
import logging
import multiprocessing
import options.iex
import os
import psycopg2
import psycopg2.extras
from options import util

root = logging.getLogger()
root.setLevel(logging.INFO)


def update_eod_options(conn, iex):

    sql = """
    SELECT d.symbol, d.ex_date
    FROM universe.dividends d
    INNER JOIN universe.eod_prices p
    ON d.symbol = p.symbol
    LEFT JOIN (
        SELECT symbol, max(db_updated) as latest_update
        FROM universe.eod_call_options
        GROUP BY symbol) o
    ON d.symbol = o.symbol
    WHERE
        (d.gross_annual_yield / p.previous_stock_price) > 0.02 AND
        p.previous_stock_price > 7.5 AND
        d.ex_date > NOW() AND
        (o.latest_update IS NULL OR o.latest_update < (NOW() - INTERVAL '12 hours'));
    """

    update_sql = """
    INSERT INTO universe.eod_call_options(
        symbol, id, expiration_date, contract_size, strike_price, closing_price, last_updated,
        type, volume, bid, ask, is_adjusted
    ) VALUES (
        %(symbol)s,
        %(id)s,
        %(expiration_date)s,
        %(contract_size)s,
        %(strike_price)s,
        %(closing_price)s,
        %(last_updated)s,
        %(type)s,
        %(volume)s,
        %(bid)s,
        %(ask)s,
        %(is_adjusted)s
    )
    ON CONFLICT(id)
    DO UPDATE SET
        symbol = EXCLUDED.symbol,
        id = EXCLUDED.id,
        expiration_date = EXCLUDED.expiration_date,
        contract_size = EXCLUDED.contract_size,
        strike_price = EXCLUDED.strike_price,
        closing_price = EXCLUDED.closing_price,
        last_updated = EXCLUDED.last_updated,
        type = EXCLUDED.type,
        volume = EXCLUDED.volume,
        bid = EXCLUDED.bid,
        ask = EXCLUDED.ask,
        is_adjusted = EXCLUDED.is_adjusted
    """

    with conn.cursor() as update_cursor:
        with conn.cursor(cursor_factory=psycopg2.extras.RealDictCursor) as cursor:
            cursor.execute('TRUNCATE universe.eod_call_options CASCADE;')
            cursor.execute(sql)

            rows = [row for row in cursor.fetchall()]

            with multiprocessing.Pool(2) as p:
                params = p.map(_process, rows)
            flattened_params = [p for symbol_params in params for p in symbol_params]
            update_cursor.executemany(update_sql, flattened_params)


def _process(row):
    symbol, min_contract_date = row['symbol'], datetime.datetime.strftime(row['ex_date'], '%Y%m')

    dates = iex.get_call_expiration_dates(symbol)
    params = []

    for expiration_date in dates:
        # TODO: remove 2021 requirement
        if expiration_date >= min_contract_date and expiration_date[:4] == '2021':
            results = iex.get_calls(symbol, expiration_date)
            if results:
                params.extend([{util.to_snake(k): v for k, v in instance.items()} for instance in results])
    return params


if __name__ == '__main__':
    iex = options.iex.IEX()
    iex.token = os.getenv('IEX_TOKEN')

    with psycopg2.connect(dbname=os.getenv('DB_NAME'),
                          user=os.getenv('DB_USER'),
                          password=os.getenv('DB_PASS'),
                          host=os.getenv('DB_HOST'),
                          port=os.getenv('DB_PORT')) as conn:
        update_eod_options(conn, iex)
