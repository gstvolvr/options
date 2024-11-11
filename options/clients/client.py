"""
This repository is a graveyard of data providers and APIs

Instead of refactoring every time I need to switch providers I create a simple API that each client adheres to
"""

from datetime import datetime
import logging
import requests
import typing

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


class Client:
    def __init__(self):
        pass
    def get_last_trading_date(self) -> typing.Optional[str]:
        """
        Expected structure:
            {
                ...
            }
        """
        pass
    def get_last_dividend(self, ticker: str) -> typing.Optional[str]:
        """
        Expected structure:
            {
                ...
            }
        """
        pass
    def get_next_dividend(self, ticker: str) -> typing.Optional[str]:
        """
        Expected structure:
            {
                ...
            }
        """
        pass