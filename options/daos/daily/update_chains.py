import logging
import json
import options.td_ameritrade
import os
import csv
import time
import datetime
from options import util

root = logging.getLogger()
root.setLevel(logging.INFO)


def update_eod_options(data_path):

    writer = None
    n_empty_params, n_faulty_date_params = 0, 0
    count = 0
    with open(f'{data_path}/options.csv', 'w') as w:
        with open(f'{data_path}/quotes.json', 'r') as f:
            quotes = json.load(f)

            for symbol, quote in quotes.items():
                count += 1
                logging.info(f'getting options chain for: {symbol}')
                try:
                    symbol_params = _process(symbol=symbol, item=quote)

                except Exception as e:
                    continue

                if not symbol_params:
                    n_empty_params += 1

                for date_params in symbol_params:
                    if writer is None:
                        writer = csv.DictWriter(w, fieldnames=date_params.keys())
                        writer.writeheader()
                    if date_params['ask'] != 0:
                        writer.writerow(date_params)
                    else:
                        n_faulty_date_params += 1

    logging.info(f'number of empty outputs: {n_empty_params}')
    logging.info(f'number of faulty options: {n_faulty_date_params}')


def _process(symbol, item):
    min_contract_date = item['dividend_ex_date']
    response = client.get_chains(ticker=symbol, from_date=min_contract_date)
    data = []

    for expiration_date in response.get('callExpDateMap', []):
        for strike in response['callExpDateMap'].get(expiration_date, []):
            for element in response['callExpDateMap'][expiration_date].get(strike, []):
                record = {util.to_snake(k): v for k, v in element.items()}
                record['option_symbol'] = record['symbol']
                record['expiry'] = expiration_date
                record['strike'] = strike

                record.update({c: response['underlying'][c] for c in ['symbol', 'close', 'last']})
                record['quote_date'] = datetime.datetime.fromtimestamp(int(response['underlying']['quoteTime'] / 1000))
                record['quote_date'] = datetime.datetime.strftime(record['quote_date'], '%Y-%m-%d')
                data.append(record)

    return data


if __name__ == '__main__':
    client = options.td_ameritrade.TDAmeritrade()
    client.token = os.getenv('TD_AMERITRADE_TOKEN')
    update_eod_options(data_path=os.getenv('DATA_PATH'))
