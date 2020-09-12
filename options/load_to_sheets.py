from google.oauth2 import service_account
from googleapiclient.discovery import build
import os
import os.path
import csv
import collections


def main(data_path):

    SPREADSHEET_ID = os.getenv('QA_GOOGLE_SHEETS_ID')
    SCOPES = ['https://www.googleapis.com/auth/spreadsheets']
    SECRET_PATH = os.getenv('GOOGLE_SHEETS_CLIENT_SECRET')

    cols = collections.OrderedDict({
        'symbol': str,
        'company_name': str,
        'industry': str,
        'previous_stock_price': float,
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
        'bid': float,
        'mid': float,
        'ask': float,
        'previous_date': str})

    with open(f'{data_path}/companies.csv', 'r') as f:
        companies = {row['symbol']: row for row in csv.DictReader(f)}

    values = []
    with open(f'{data_path}/returns.csv', 'r') as f:
        returns = csv.DictReader(f)

        for row in returns:
            ordered_result = []
            for col, type_func in cols.items():
                if col in ['company_name', 'industry']:
                    ordered_result.append(companies[row['symbol']][col])
                else:
                    value = type_func(row[col]) if row[col] else ''
                    ordered_result.append(value)
            values.append(ordered_result)

    creds = service_account.Credentials.from_service_account_file(SECRET_PATH, scopes=SCOPES)
    RANGE_NAME= 'data'

    service = build('sheets', 'v4', credentials=creds, cache_discovery=False)
    service.spreadsheets().values().clear(spreadsheetId=SPREADSHEET_ID, range=RANGE_NAME).execute()
    body = {'values': [list(cols.keys())] + list(map(list, values))}
    service.spreadsheets().values().update(
        spreadsheetId=SPREADSHEET_ID, range=RANGE_NAME, valueInputOption='RAW', body=body).execute()


if __name__ == '__main__':
    main(data_path=os.getenv('DATA_PATH'))
