from google.oauth2 import service_account
from googleapiclient.discovery import build
from google.cloud import secretmanager
import os.path
import csv
import collections
import gc
import time
import socket
import logging
import math
import json
from functools import lru_cache

socket.setdefaulttimeout(300)

BATCH_SIZE = 500  # Increased batch size for fewer API calls
MAX_RETRIES = 3
SCOPES = ['https://www.googleapis.com/auth/spreadsheets']
SECRET_PATH = os.getenv('GOOGLE_SHEETS_CLIENT_SECRET')
SLEEP_SECONDS = 0.5  # Reduced sleep time between API calls

def get_secret(secret_name, project_id="options-282500"):
    """Retrieve a secret from Google Cloud Secret Manager"""
    client = secretmanager.SecretManagerServiceClient()
    name = f"projects/{project_id}/secrets/{secret_name}/versions/latest"
    response = client.access_secret_version(request={"name": name})
    return response.payload.data.decode("UTF-8")

# SPREADSHEET_ID will be set in main() function


def clean_value(value):
    """Clean values before sending to Google Sheets"""
    if isinstance(value, float):
        if math.isinf(value) or math.isnan(value):
            return ''
    return value

def upload_to_sheets(service, body, row_number, range_name, spreadsheet_id):
    """Upload data to Google Sheets with retry logic"""
    try_n, try_again = 0, True

    while try_again:
        try:
            service.spreadsheets().values().update(
                spreadsheetId=spreadsheet_id,
                range=range_name.format(row_number=row_number),
                valueInputOption='RAW',
                body=body).execute()
            try_again = False
        except socket.timeout:
            logging.warn(f'socket timeout....retry # {try_n}')
            try_n += 1
            try_again = try_n < MAX_RETRIES

            if try_n == MAX_RETRIES:
                raise Exception('Reached max retries')

    return row_number + len(body['values'])


def main(data_path):
    # Get Google Sheets ID from Secret Manager
    try:
        spreadsheet_id = get_secret('GOOGLE_SHEETS_ID')
    except Exception as e:
        logging.error(f"Failed to get Google Sheets ID from Secret Manager: {e}")
        # Fallback to environment variable
        spreadsheet_id = os.getenv('GOOGLE_SHEETS_ID')
        if not spreadsheet_id:
            raise Exception("No Google Sheets ID found in Secret Manager or environment variables")

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
    })

    values = []

    # Get credentials from Secret Manager instead of file
    try:
        service_account_info = json.loads(get_secret('SERVICE_ACCOUNT_KEY'))
        creds = service_account.Credentials.from_service_account_info(service_account_info, scopes=SCOPES)
    except Exception as e:
        # Fallback to file-based credentials if Secret Manager fails
        logging.warning(f"Failed to get credentials from Secret Manager: {e}, falling back to file")
        creds = service_account.Credentials.from_service_account_file(SECRET_PATH, scopes=SCOPES)
    SHEET_NAME = 'data'
    RANGE_NAME= f'{SHEET_NAME}!A{{row_number}}'

    service = build('sheets', 'v4', credentials=creds, cache_discovery=False)
    # clear current data in the spreadsheet
    service.spreadsheets().values().clear(spreadsheetId=spreadsheet_id, range=SHEET_NAME).execute()

    # Upload headers with retry logic
    body = {'values': [list(cols.keys())]}
    row_number = upload_to_sheets(service, body, 1, RANGE_NAME, spreadsheet_id)
    with open(f'{data_path}/schwab_returns.csv', 'r') as f:
        returns = csv.DictReader(f)

        for i, row in enumerate(returns):
            ordered_result = []
            for col, type_func in cols.items():
                value = type_func(row[col]) if col in row and row[col] else ''
                ordered_result.append(clean_value(value))
            values.append(ordered_result)

            # Only sort once before uploading
            values = sorted(values, key=lambda r: (r[0], r[6], r[5]))
            body = {'values': list(map(list, values))}

            if i % BATCH_SIZE == 0 and i != 0:

                # Use the upload_to_sheets function with retry logic
                try:
                    row_number = upload_to_sheets(service, body, row_number, RANGE_NAME, spreadsheet_id)
                except Exception as e:
                    logging.error(f"Error uploading batch: {e}")
                    logging.error(f"Problem row data: {body['values'][-1]}")
                    # continue

                values = []
                # Only collect garbage every few batches to reduce overhead
                if i % (BATCH_SIZE * 4) == 0:
                    gc.collect()
                # see usage limits: https://developers.google.com/sheets/api/limits
                time.sleep(SLEEP_SECONDS)
        # Sort and upload the final batch with retry logic
        print("finished here")
        try:
            upload_to_sheets(service, body, row_number, RANGE_NAME, spreadsheet_id)
        except Exception as e:
            logging.error(f"Error uploading batch: {e}")
            logging.error(f"Problem row data: {body['values'][-1]}")


if __name__ == '__main__':
    main(data_path=os.getenv('DATA_PATH'))
