import logging
import options.iex
import os
import csv

root = logging.getLogger()
root.setLevel(logging.INFO)

BATCH_SIZE = 500


def update_membership(data_path):
    logging.info('starting to update universe.symbols')

    symbols = iex.get_all_symbols()

    with open(f'{data_path}/universe.csv', 'w') as w:
        writer = csv.writer(w)
        for i, params in enumerate(symbols):
            symbol = params['symbol']
            if (i != 0) and (i % 100) == 0:
                logging.info(f'processed: {i}')

            if (i != 0) and (i % BATCH_SIZE) == 0:
                logging.info(f'batch {i / BATCH_SIZE}')
            in_universe = iex.has_dividends(symbol) and iex.has_options(symbol)

            if in_universe:
                writer.writerow([symbol])


if __name__ == '__main__':
    iex = options.iex.IEX()
    iex.token = os.getenv('IEX_TOKEN')
    update_membership(os.getenv('DATA_PATH'))
