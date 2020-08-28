import logging
import psycopg2
import options.iex
import re
import os

root = logging.getLogger()
root.setLevel(logging.INFO)

BATCH_SIZE = 500


def to_snake(name):
    return re.sub(f'(?<!^)(?=[A-Z])', '_', name).lower()


def update_companies(conn, iex):
    logging.info('starting to get company info')

    sql = """
    SELECT symbol 
    FROM universe.symbols 
    WHERE in_universe = TRUE
    ORDER BY symbol
    """

    update_sql = """
    INSERT INTO universe.companies(
        symbol, company_name, employees, exchange, industry, description, sector, issue_type, tags, state, country
    ) VALUES (
        %(symbol)s, 
        %(company_name)s, 
        %(employees)s, 
        %(exchange)s, 
        %(industry)s, 
        %(description)s, 
        %(sector)s, 
        %(issue_type)s,
        %(tags)s, 
        %(state)s, 
        %(country)s
    )
    ON CONFLICT(symbol)
    DO UPDATE SET
        symbol = EXCLUDED.symbol, 
        company_name = EXCLUDED.company_name, 
        employees = EXCLUDED.employees,
        exchange = EXCLUDED.exchange, 
        industry = EXCLUDED.industry, 
        description = EXCLUDED.description, 
        sector = EXCLUDED.sector, 
        issue_type = EXCLUDED.issue_type,
        tags = EXCLUDED.tags, 
        state = EXCLUDED.state, 
        country = EXCLUDED.country
    """

    with conn.cursor() as update_cursor:
        with conn.cursor() as cursor:
            cursor.execute(sql)

            for i, (symbol,) in enumerate(cursor.fetchall()):
                if (i != 0) and (i % 100) == 0:
                    logging.info(f'processed: {i}')
                company = {to_snake(k): v for k, v in iex.get_company(symbol).items()}
                update_cursor.execute(update_sql, company)


if __name__ == '__main__':
    iex = options.iex.IEX()
    iex.token = os.getenv('IEX_TOKEN')

    with psycopg2.connect(dbname=os.getenv('DB_NAME'),
                          user=os.getenv('DB_USER'),
                          password=os.getenv('DB_PASS'),
                          host=os.getenv('DB_HOST'),
                          port=os.getenv('DB_PORT')) as conn:
        update_companies(conn, iex)
