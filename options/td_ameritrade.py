from datetime import datetime
import logging
import requests


REQUEST_DATE_FORMAT = '%Y%m%d'


def _get(url, json=True):
    try:
        response = requests.get(url)
        data = response.json() if json else response.text
    except Exception as e:
        logging.debug(f'ERROR: {e}')
        if json:
            data = {}
        else:
            data = None
    return data


def _convert_str_format(date, current_date_format):
    return datetime.strptime(date, current_date_format).strftime(REQUEST_DATE_FORMAT)


class TDAmeritrade:
    def __init__(self):
        self.token = None
        self.base_url = 'https://api.tdameritrade.com/v1'

    def get_chains(self, ticker: str, from_date: str):
        return _get(f'{self.base_url}/marketdata/chains?apikey={self.token}&symbol={ticker}&contractType=CALL&includeQuotes=TRUE&optionType=S&fromDate={from_date}')

    def get_quote(self, ticker: str):
        return _get(f'{self.base_url}/marketdata/quotes?apikey={self.token}&symbol={ticker}')
