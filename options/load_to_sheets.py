from google.oauth2 import service_account
from googleapiclient.discovery import build
import os
import os.path
import csv


def main(data_path):

    SPREADSHEET_ID = os.getenv('QA_GOOGLE_SHEETS_ID')
    SCOPES = ['https://www.googleapis.com/auth/spreadsheets']
    SECRET_PATH = os.getenv('GOOGLE_SHEETS_CLIENT_SECRET')

    cols = [
        'symbol',
        'company_name',
        'industry',
        'previous_stock_price',
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

    with open(f'{data_path}/companies.csv', 'r') as f:
        companies = {row['symbol']: row for row in csv.DictReader(f)}

    values = []
    with open(f'{data_path}/returns.csv', 'r') as f:
        returns = csv.DictReader(f)

        for row in returns:
            ordered_result = []
            for col in cols:
                if col in ['company_name', 'industry']:
                    ordered_result.append(companies[row['symbol']][col])
                else:
                    ordered_result.append(row[col])
            values.append(ordered_result)

    creds = service_account.Credentials.from_service_account_file(SECRET_PATH, scopes=SCOPES)
    RANGE_NAME= 'data'

    service = build('sheets', 'v4', credentials=creds, cache_discovery=False)
    service.spreadsheets().values().clear(spreadsheetId=SPREADSHEET_ID, range=RANGE_NAME).execute()
    body = {'values': [cols] + list(map(list, values))}
    service.spreadsheets().values().update(
        spreadsheetId=SPREADSHEET_ID, range=RANGE_NAME, valueInputOption='RAW', body=body).execute()


if __name__ == '__main__':
    main(data_path=os.getenv('DATA_PATH'))
