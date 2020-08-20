import pandas as pd
import pickle
import os.path
import os
import psycopg2
from googleapiclient.discovery import build
from google.auth.transport.requests import Request
from google.oauth2 import service_account


def main(conn):

    SPREADSHEET_ID = os.getenv('GOOGLE_SHEETS_ID')
    SCOPES = ['https://www.googleapis.com/auth/spreadsheets']
    SECRET_PATH = os.getenv('GOOGLE_SHEETS_CLIENT_SECRET')

    cols = [
        'symbol',
        'company_name',
        'industry',
        'stock_price',
        'net',
        'strike_price',
        'expiration_date',
        'insurance',
        'premium',
        'dividend_amount',
        'dividend_ex_date',
        'return_after_1_div',
        'return_after_2_div',
        'return_after_3_div',
        'bid',
        'mid',
        'ask',
        'previous_date']

    sql = """
        SELECT
            r.symbol,
            c.company_name,
            c.industry,
            p.previous_stock_price as stock_price,
            r.net,
            o.strike_price,
            to_char(o.expiration_date, 'YYYY-MM-DD') as expiration_date,
            r.insurance,
            r.premium,
            d.amount as dividend_amount,
            to_char(d.ex_date, 'YYYY-MM-DD') as dividend_ex_date,
            r.return_after_1_div,
            r.return_after_2_div,
            r.return_after_3_div,
            o.bid,
            r.mid,
            o.ask,
            to_char(p.previous_date, 'YYYY-MM-DD') as previous_date
        FROM universe.returns r
        LEFT JOIN universe.companies c
        ON r.symbol = c.symbol
        LEFT JOIN universe.eod_call_options o
        ON r.id = o.id
        LEFT JOIN universe.eod_prices p
        ON o.symbol = p.symbol AND o.last_updated = p.previous_date
        LEFT JOIN universe.dividends d
        ON r.symbol = d.symbol
        WHERE r.return_after_1_div IS NOT NULL and p.previous_stock_price IS NOT NULL
        ORDER BY r.symbol, o.expiration_date, o.strike_price;
    """

    with conn.cursor() as cursor:
        cursor.execute(sql)
        values = cursor.fetchall()

    creds = None
    # The file token.pickle stores the user's access and refresh tokens, and is
    # created automatically when the authorization flow completes for the first
    # time.
    if os.path.exists('token.pickle'):
        with open('token.pickle', 'rb') as token:
            creds = pickle.load(token)
    # If there are no (valid) credentials available, let the user log in.
    if not creds or not creds.valid:
        if creds and creds.expired and creds.refresh_token:
            creds.refresh(Request())
        else:
            creds = service_account.Credentials.from_service_account_file(SECRET_PATH, scopes=SCOPES)
            #creds = flow.run_local_server(port=0)
        # Save the credentials for the next run
        #with open('token.pickle', 'wb') as token:
        #    pickle.dump(creds, token)

    RANGE_NAME= 'data'

    service = build('sheets', 'v4', credentials=creds, cache_discovery=False)
    service.spreadsheets().values().clear(spreadsheetId=SPREADSHEET_ID, range=RANGE_NAME).execute()
    body = {'values': [cols] + list(map(list, values))}
    service.spreadsheets().values().update(
        spreadsheetId=SPREADSHEET_ID, range=RANGE_NAME, valueInputOption='RAW', body=body).execute()


if __name__ == '__main__':
    print(os.getenv('DB_NAME'))
    print(os.getenv('DB_USER'))
    with psycopg2.connect(dbname=os.getenv('DB_NAME'),
                          user=os.getenv('DB_USER'),
                          password=os.getenv('DB_PASS'),
                          host=os.getenv('DB_HOST'),
                          port=os.getenv('DB_PORT')) as conn:
        main(conn)
