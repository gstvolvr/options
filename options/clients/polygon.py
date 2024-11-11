from typing import Optional
from multiprocessing import current_process

import logging

from six import Iterator

from polygon.rest.models.snapshot import OptionContractSnapshot
import options.clients.client
import polygon
import datetime
import typing

DUMMY_TICKER = 'SPY'

"""
Help reduce overhead of creating clients for processes
"""
class LazyClient:
    _instances = {}

    @classmethod
    def get_instance(cls):
        process_name = current_process().name
        if process_name not in cls._instances:
            logging.info(f"Initializing Polygon client in process {process_name}")
            cls._instances[process_name] = Polygon()
        return cls._instances[process_name]


class Polygon(options.clients.client.Client):
    def __init__(self):
        super().__init__()
        self.client = polygon.RESTClient()

    def get_last_trading_date(self) -> typing.Optional[str]:
        previous_closes = self.client.get_previous_close_agg(DUMMY_TICKER)
        for previous_close in previous_closes:
            last_open_date = datetime.datetime.fromtimestamp(previous_close.timestamp / 1000)
            return last_open_date.strftime('%Y-%m-%d')

    def get_last_dividend(self, ticker: str) -> typing.Optional[dict]:
        for d in self.client.list_dividends(ticker):
            return d.__dict__


    def get_next_dividend(self, ticker: str) -> typing.Optional[dict]:
        today = datetime.datetime.today().strftime('%Y-%m-%d')
        for d in self.client.list_dividends(ticker, ex_dividend_date_gte=today):
            return d.__dict__

    def get_close_price(self, ticker: str, date: str) -> typing.Optional[dict]:
        for a in self.client.get_aggs(ticker=ticker, multiplier=1, timespan="minute", from_=date, to=date):
            return a.__dict__

    def get_contracts(self, ticker: str, min_contract_date: str) -> Iterator:
        return (c['ticker'] for c  in self.client.list_options_contracts(
            underlying_ticker=ticker,
            contract_type="call",
            expiration_date_gt=min_contract_date
        ))

    def get_chains(self, ticker: str) -> Iterator:
        """
        OptionsContract(additional_underlyings=None, cfi='OCASPS', contract_type='call', correction=None, exercise_style='american', expiration_date='2024-11-15', primary_exchange='BATO', shares_per_contract=100, strike_price=5, ticker='O:AAPL241115C00005000', underlying_ticker='AAPL')
        """
        def _convert(snapshot: OptionContractSnapshot) -> dict:
            record = {
                'ticker': ticker,
                'strike': snapshot.details.strike_price,
                'expiry': snapshot.details.expiration_date,
                'option_ticker': snapshot.details.ticker,
                'quote_date': snapshot.day.last_updated,
                'close': snapshot.day.close
            }
            if record['quote_date'] is not None:
                quote_date = datetime.datetime.fromtimestamp(int(record['quote_date']) // 1e9)
                record['quote_date'] = datetime.datetime.strftime(quote_date, '%Y-%m-%d')
            return record
        return (_convert(c) for c in self.client.list_snapshot_options_chain(underlying_asset=ticker))