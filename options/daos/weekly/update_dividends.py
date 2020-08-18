import datetime
import logging
import options.iex
import os
import psycopg2
import re
from dateutil.relativedelta import relativedelta
from options import util

root = logging.getLogger()
root.setLevel(logging.INFO)


def to_snake(name):
    return re.sub(f'(?<!^)(?=[A-Z])', '_', name).lower()


def update_dividends(conn, iex):
    sql = """
    SELECT symbol 
    FROM universe.symbols 
    WHERE in_universe = TRUE
    ORDER BY symbol
    """

    update_sql = """
    INSERT INTO universe.dividends(
        symbol,
        ex_date,
        payment_date,
        record_date,
        declared_date,
        amount,
        currency,
        description,
        frequency,
        calculated,
        gross_annual_yield
    ) VALUES (
        %(symbol)s,
        %(ex_date)s,
        %(payment_date)s,
        %(record_date)s,
        %(declared_date)s,
        %(amount)s,
        %(currency)s,
        %(description)s,
        %(frequency)s,
        %(calculated)s,
        %(gross_annual_yield)s
    )
    ON CONFLICT(symbol)
    DO UPDATE SET
        ex_date = EXCLUDED.ex_date,
        payment_date = EXCLUDED.payment_date,
        record_date = EXCLUDED.record_date,
        declared_date = EXCLUDED.declared_date,
        amount = EXCLUDED.amount,
        currency = EXCLUDED.currency,
        description = EXCLUDED.description,
        frequency = EXCLUDED.frequency,
        calculated = EXCLUDED.calculated,
        gross_annual_yield = EXCLUDED.gross_annual_yield
    """

    def _clean(value):
        if value == '':
            return None
        return value

    def _add_months(date, months=3):
        new_date = datetime.datetime.strptime(date, '%Y-%m-%d') + relativedelta(months=months)
        return datetime.datetime.strftime(new_date, '%Y-%m-%d')

    with conn.cursor() as update_cursor:
        with conn.cursor() as cursor:
            cursor.execute(sql)

            for i, (symbol,) in enumerate(cursor.fetchall()):
                if (i != 0) and (i % 100) == 0:
                    logging.info(f'processed: {i}')

                params = {'symbol': symbol, 'calculated': False}
                dividend = iex.get_next_dividend(symbol)

                if dividend == {}:
                    params['calculated'] = True
                    dividend = iex.get_last_dividend(symbol)
                    if 'frequency' not in dividend or dividend['frequency'] not in util.FREQUENCY_MAPPING:
                        logging.warning(f'{symbol}: {dividend}')
                        continue
                    dividend['exDate'] = _add_months(dividend['exDate'], util.FREQUENCY_MAPPING[dividend['frequency']])

                # TODO: update dividends
                dividend_clean = {to_snake(k): _clean(v) for k, v in dividend.items()}
                dividend_clean['gross_annual_yield'] = dividend_clean['amount'] * \
                                                       (util.FREQUENCY_MAPPING[dividend['frequency']] / 12.)

                # ignore non-cash dividends
                if dividend['flag'] != 'Cash':
                    continue

                params.update(dividend_clean)
                update_cursor.execute(update_sql, params)


if __name__ == '__main__':
    iex = options.iex.IEX()
    iex.token = os.getenv('IEX_TOKEN')

    with psycopg2.connect(dbname=os.getenv('DB_NAME'),
                          user=os.getenv('DB_USER'),
                          password=os.getenv('DB_PASS'),
                          host=os.getenv('DB_HOST'),
                          port=os.getenv('DB_PORT')) as conn:
        update_dividends(conn, iex)
