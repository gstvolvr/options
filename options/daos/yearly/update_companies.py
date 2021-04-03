import logging
import options.iex
import re
import os
import csv

root = logging.getLogger()
root.setLevel(logging.INFO)

BATCH_SIZE = 500


def to_snake(name):
    return re.sub(f'(?<!^)(?=[A-Z])', '_', name).lower()


def update_companies(data_path):
    logging.info('starting to get company info')

    writer = None
    with open(f'{data_path}/companies.csv', 'w') as w:
        with open(f'{data_path}/universe.csv', 'r') as f:
            symbols = [symbol.strip() for symbol in f.readlines()]
            for i, symbol in enumerate(symbols):
                if (i != 0) and (i % 100) == 0:
                    logging.info(f'processed: {i}')
                company = {to_snake(k): v for k, v in iex.get_company(symbol).items()}
                if writer is None:
                    writer = csv.DictWriter(w, fieldnames=company.keys())
                    writer.writeheader()
                writer.writerow(company)



if __name__ == '__main__':
    iex = options.iex.IEX()
    iex.token = os.getenv('IEX_TOKEN')
    update_companies(os.getenv('DATA_PATH'))
