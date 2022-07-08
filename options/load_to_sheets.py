import collections
import csv
import gc
import os.path
import psycopg2
import psycopg2.extras
import socket
import time

socket.setdefaulttimeout(300)

BATCH_SIZE = 500
MAX_RETRIES = 3
SLEEP_SECONDS = 0.75


UPDATE_TABLE = """
    INSERT INTO universe.returns(
        symbol,
        company_name,
        industry,
        last,
        net,
        strike_price,
        expiration_date,
        insurance,
        premium,
        dividend_amount,
        dividend_ex_date,
        return_after_1_div,
        return_after_2_div,
        return_after_3_div,
        return_after_4_div,
        bid,
        mid,
        ask,
        quote_date
    ) VALUES (
        %(symbol)s,
        %(company_name)s,
        %(industry)s,
        %(last)s,
        %(net)s,
        %(strike_price)s,
        %(expiration_date)s,
        %(insurance)s,
        %(premium)s,
        %(dividend_amount)s,
        %(dividend_ex_date)s,
        %(return_after_1_div)s,
        %(return_after_2_div)s,
        %(return_after_3_div)s,
        %(return_after_4_div)s,
        %(bid)s,
        %(mid)s,
        %(ask)s,
        %(quote_date)s
    ) ON CONFLICT (symbol, strike_price, expiration_date) DO UPDATE
    SET
        symbol=EXCLUDED.symbol,
        company_name=EXCLUDED.company_name,
        industry=EXCLUDED.industry,
        last=EXCLUDED.last,
        net=EXCLUDED.net,
        strike_price=EXCLUDED.strike_price,
        expiration_date=EXCLUDED.expiration_date,
        insurance=EXCLUDED.insurance,
        premium=EXCLUDED.premium,
        dividend_amount=EXCLUDED.dividend_amount,
        dividend_ex_date=EXCLUDED.dividend_ex_date,
        return_after_1_div=EXCLUDED.return_after_1_div,
        return_after_2_div=EXCLUDED.return_after_2_div,
        return_after_3_div=EXCLUDED.return_after_3_div,
        return_after_4_div=EXCLUDED.return_after_4_div,
        bid=EXCLUDED.bid,
        mid=EXCLUDED.mid,
        ask=EXCLUDED.ask,
        quote_date=EXCLUDED.quote_date
"""


def main(data_path):

    cols = collections.OrderedDict({
        'symbol': str,
        'company_name': str,
        'industry': str,
        'last': float,
        'net': float,
        'strike_price': float,
        'expiration_date': str,
        'insurance': float,
        'premium': float,
        'dividend_amount': float,
        'dividend_ex_date': str,
        'return_after_1_div': float,
        'return_after_2_div': float,
        'return_after_3_div': float,
        'return_after_4_div': float,
        'bid': float,
        'mid': float,
        'ask': float,
        'quote_date': str})

    with open(f'{data_path}/companies.csv', 'r') as f:
        companies = {row['symbol']: row for row in csv.DictReader(f)}

    with psycopg2.connect(database='postgres', user='postgres', host='10.60.176.3', port=5432) as conn:
        with conn.cursor(cursor_factory=psycopg2.extras.DictCursor) as cursor:
            with open(f'{data_path}/returns.csv', 'r') as f:
                returns = csv.DictReader(f)

                values = []
                for i, row in enumerate(returns):
                    result = {}
                    for col, type_func in cols.items():
                        if col in ['company_name', 'industry']:
                            if row['symbol'] in companies:
                                result[col] = companies[row['symbol']][col]
                            else:
                                result[col] = None
                        else:
                            result[col] = type_func(row[col]) if row[col] else None
                    values.append(result)

                    if i % BATCH_SIZE == 0 and i != 0:
                        print(i)
                        cursor.executemany(UPDATE_TABLE, values)
                        conn.commit()
                        values = []
                        gc.collect()
                cursor.executemany(UPDATE_TABLE, values)

if __name__ == '__main__':
    main(data_path=os.getenv('DATA_PATH'))
