import logging
import psycopg2
import psycopg2.extras
import options.iex
import os
from options import util

root = logging.getLogger()
root.setLevel(logging.INFO)


def update_returns(conn, iex):
    sql = """
    SELECT 
        o.id,
        o.symbol,
        o.strike_price,
        o.expiration_date::TIMESTAMP,
        o.bid,
        o.ask,
        o.is_adjusted,
        p.previous_stock_price,
        p.previous_stock_price,
        d.amount as dividend_amount,
        d.ex_date::TIMESTAMP as dividend_ex_date,
        d.frequency as dividend_frequency
    FROM universe.eod_call_options o
    INNER JOIN universe.eod_prices p
    on o.symbol = p.symbol AND o.last_updated = p.previous_date
    INNER JOIN universe.dividends d
    on o.symbol = d.symbol
    WHERE 
        o.ask != 0 AND 
        o.is_adjusted = FALSE AND
        d.frequency IS NOT NULL AND
        d.frequency NOT IN ('final', 'unspecified')
    ORDER BY id
    """

    update_sql = """
    INSERT INTO universe.returns(
          symbol,
          id, 
          net, 
          premium,
          insurance,
          return_after_1_div,
          return_after_2_div,
          return_after_3_div
    ) VALUES (
        %(symbol)s,
        %(id)s,
        %(net)s, 
        %(premium)s,
        %(insurance)s,
        %(return_after_1_div)s,
        %(return_after_2_div)s,
        %(return_after_3_div)s
    ) ON CONFLICT(id)
    DO UPDATE SET
        symbol = EXCLUDED.symbol,
        id = EXCLUDED.id, 
        net = EXCLUDED.net, 
        premium = EXCLUDED.premium,
        insurance  = EXCLUDED.insurance,
        return_after_1_div = EXCLUDED.return_after_1_div,
        return_after_2_div = EXCLUDED.return_after_2_div,
        return_after_3_div = EXCLUDED.return_after_3_div
    """

    with conn.cursor() as update_cursor:
        with conn.cursor(cursor_factory=psycopg2.extras.RealDictCursor) as cursor:
            cursor.execute(sql)

            for i, (row) in enumerate(cursor.fetchall()[:50]):
                row = dict(row)
                if (i != 0) and (i % 100) == 0:
                    logging.info(f'processed: {i}')

                row['mid'] = (row['bid'] + row['ask']) / 2
                row['net'] = (row['previous_stock_price'] - row['mid'])
                row['premium'] = row['strike_price'] - row['net']
                row['insurance'] = (row['previous_stock_price'] - row['net']) / row['previous_stock_price']

                # ignore unrealistic premiums
                if row['premium'] < 0.05:
                    continue

                for j in range(0, 6):
                    row[f'return_after_{j+1}_div'] = util.days_to_next_event(row, i=j)
                update_cursor.execute(update_sql, row)


if __name__ == '__main__':
    iex = options.iex.IEX()
    iex.token = os.getenv('IEX_TOKEN')

    with psycopg2.connect(dbname=os.getenv('DB_NAME'),
                          user=os.getenv('DB_USER'),
                          password=os.getenv('DB_PASS'),
                          host=os.getenv('DB_HOST'),
                          port=os.getenv('DB_PORT')) as conn:
        update_returns(conn, iex)

