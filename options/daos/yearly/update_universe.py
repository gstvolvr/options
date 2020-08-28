import logging
import psycopg2
import options.iex
import os

root = logging.getLogger()
root.setLevel(logging.INFO)


BATCH_SIZE = 500


def populate_symbols_table(conn, iex):
    logging.info('starting to populate symbols table')
    symbols = iex.get_all_symbols()

    sql = """
    INSERT INTO universe.symbols(symbol, date)
    VALUES(
        %(symbol)s, 
        %(date)s
    )
    ON CONFLICT(symbol)
    DO UPDATE SET
        symbol = EXCLUDED.symbol,
        date = EXCLUDED.date
    """

    with conn.cursor() as cursor:
        cursor.executemany(sql, symbols)
    logging.info('Done loading symbols.')


def update_membership(conn, iex):
    sql = """
    SELECT symbol 
    FROM universe.symbols 
    ORDER BY symbol
    """

    update_sql = """
    UPDATE universe.symbols
    SET in_universe = %(in_universe)s
    WHERE symbol = %(symbol)s
    """
    logging.info('starting to update universe.symbols')

    with conn.cursor() as update_cursor:
        params = []
        with conn.cursor() as cursor:
            cursor.execute(sql)

            for i, (symbol,) in enumerate(cursor.fetchall()):
                if (i != 0) and (i % 100) == 0:
                    logging.info(f'processed: {i}')

                if (i != 0) and (i % BATCH_SIZE) == 0:
                    logging.info(f'batch {i / BATCH_SIZE} of size {len(params)} done.')
                    update_cursor.executemany(update_sql, params)
                    params = []
                in_universe = iex.has_dividends(symbol) and iex.has_options(symbol)
                params.append({'symbol': symbol, 'in_universe': in_universe})
        logging.info(f'last batch of size {len(params)} done.')
        update_cursor.executemany(update_sql, params)


if __name__ == '__main__':
    iex = options.iex.IEX()
    iex.token = os.getenv('IEX_TOKEN')

    with psycopg2.connect(dbname=os.getenv('DB_NAME'),
                          user=os.getenv('DB_USER'),
                          password=os.getenv('DB_PASS'),
                          host=os.getenv('DB_HOST'),
                          port=os.getenv('DB_PORT')) as conn:
        populate_symbols_table(conn, iex)
        update_membership(conn, iex)
