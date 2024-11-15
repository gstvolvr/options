from google.oauth2 import service_account
from googleapiclient.discovery import build
import os.path
import csv
import collections
import gc
import time
import socket
import logging

socket.setdefaulttimeout(300)

BATCH_SIZE = 250
MAX_RETRIES = 3
SPREADSHEET_ID = os.getenv('GOOGLE_SHEETS_ID')
SCOPES = ['https://www.googleapis.com/auth/spreadsheets']
SECRET_PATH = os.getenv('GOOGLE_SHEETS_CLIENT_SECRET')
SLEEP_SECONDS = 0.75


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
        'dividend_quarterly_amount': float,
        'dividend_ex_date': str,
        'return_after_1_div': float,
        'return_after_2_div': float,
        'return_after_3_div': float,
        'return_after_4_div': float,
        'return_after_5_div': float,
        'return_after_last_div': float,
        'bid': float,
        'mid': float,
        'ask': float,
        'quote_date': str,
        'volatility': str,
        'delta': str,
        'gamma': str,
        'theta': str,
        'vega': str,
        'rho': str,
        'time_value': str})

    with open(f'{data_path}/companies.csv', 'r') as f:
        companies = {row['symbol']: row for row in csv.DictReader(f)}

    values = []

    creds = service_account.Credentials.from_service_account_file(SECRET_PATH, scopes=SCOPES)
    SHEET_NAME = 'data'
    RANGE_NAME= f'{SHEET_NAME}!A{{row_number}}'

    service = build('sheets', 'v4', credentials=creds, cache_discovery=False)
    # clear current data in the spreadsheet
    service.spreadsheets().values().clear(spreadsheetId=SPREADSHEET_ID, range=SHEET_NAME).execute()
    body = {'values': [list(cols.keys())]}
    service.spreadsheets().values().update(
        spreadsheetId=SPREADSHEET_ID,
        range=RANGE_NAME.format(row_number=1),
        valueInputOption='RAW',
        body=body).execute()

    row_number = 2
    with open(f'{data_path}/returns.csv', 'r') as f:
        returns = csv.DictReader(f)

        for i, row in enumerate(returns):
            ordered_result = []
            for col, type_func in cols.items():
                if col in ['company_name', 'industry']:
                    if row['symbol'] in companies:
                        ordered_result.append(companies[row['symbol']][col])
                    else:
                        ordered_result.append(None)
                else:
                    value = type_func(row[col]) if row[col] else ''
                    ordered_result.append(value)
            values.append(ordered_result)

            if i % BATCH_SIZE == 0 and i != 0:
                # order by: symbol, expiration_date, strike_price
                values = sorted(values, key=lambda r: (r[0],
                                                       r[6],
                                                       r[5]))
                body = {'values': list(map(list, values))}
                try_n, try_again = 0, True

                while try_again:
                    try:
                        service.spreadsheets().values().update(
                            spreadsheetId=SPREADSHEET_ID,
                            range=RANGE_NAME.format(row_number=row_number),
                            valueInputOption='RAW',
                            body=body).execute()
                        try_again = False
                    except socket.timeout:
                        logging.warn(f'socket timeout....retry # {try_n}')
                        try_n += 1
                        try_again = try_n < MAX_RETRIES

                        if try_n == MAX_RETRIES:
                            raise Exception('Reached max retries')


                values = []
                gc.collect()
                # see usage limits: https://developers.google.com/sheets/api/limits
                row_number += BATCH_SIZE
                time.sleep(SLEEP_SECONDS)
        values = sorted(values, key=lambda r: (r[0],
                                               r[6],
                                               r[5]))
        body = {'values': list(map(list, values))}
        service.spreadsheets().values().update(
            spreadsheetId=SPREADSHEET_ID,
            range=RANGE_NAME.format(row_number=row_number),
            valueInputOption='RAW',
            body=body).execute()

if __name__ == '__main__':
    main(data_path=os.getenv('DATA_PATH'))
