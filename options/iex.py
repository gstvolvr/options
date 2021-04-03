import requests
import logging


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


class IEX:
    def __init__(self):
        self.token = None
        self.base_url = 'https://cloud.iexapis.com'
        self.stats = {}

    def get_company(self, symbol):
        data = _get(f'{self.base_url}/stable/stock/{symbol}/company?token={self.token}')
        return data or {}

    def get_quote(self, symbol):
        data = _get(f'{self.base_url}/stable/stock/{symbol}/batch?types=quote&token={self.token}')
        return data['quote']

    # TODO: remove method
    def get_industry(self, symbol):
        data = _get(f'{self.base_url}/stable/stock/{symbol}/company?token={self.token}')
        if data:
            return data['industry']

    def get_price(self, symbol, date=None):
        if date is None:
            data = _get(f'{self.base_url}/stable/stock/{symbol}/price?token={self.token}')
        else:
            price_list = _get(f'{self.base_url}/stable/stock/{symbol}/chart/1d/{date}?token={self.token}')
            data = sorted(price_list, key=lambda d: d['minute'])[-1]['marketClose']
        return data

    def get_quote(self, symbol):
        data = _get(f'{self.base_url}/stable/stock/{symbol}/quote?token={self.token}')
        if data:
            return data

    def get_stats(self, symbol):
        if symbol in self.stats:
            return self.stats[symbol]
        else:
            self.stats = {symbol: _get(f'{self.base_url}/stable/stock/{symbol}/stats?token={self.token}') or {}}
        return self.stats[symbol]

    def get_dividend_yield(self, symbol):
        symbol_stats = self.get_stats(symbol)
        if 'dividendYield' in symbol_stats:
            return symbol_stats['dividendYield']
        return None

    def has_options(self, symbol):
        dates = self.get_call_expiration_dates(symbol=symbol)
        return dates is not None and len(dates) > 0

    def has_dividends(self, symbol):
        symbol_stats = self.get_stats(symbol)
        return ('nextDividendDate' in symbol_stats and symbol_stats['nextDividendDate'] is not None) or \
               ('exDividendDate' in symbol_stats and symbol_stats['exDividendDate'] is not None)

    def get_last_dividend(self, symbol, time_frame='1y'):
        data = _get(f'{self.base_url}/stable/stock/{symbol}/dividends/{time_frame}?token={self.token}')
        if data:
            return sorted(data, key=lambda d: d['exDate'])[-1]
        return {}

    def get_next_dividend(self, symbol):
        data = _get(f'{self.base_url}/stable/stock/{symbol}/dividends/next?token={self.token}')
        if data:
            return data[0]
        return {}

    def get_call_expiration_dates(self, symbol):
        return _get(f'{self.base_url}/stable/stock/{symbol}/options?token={self.token}')

    def get_calls(self, symbol, expiration_date):
        return _get(f'{self.base_url}/stable/stock/{symbol}/options/{expiration_date}/call?token={self.token}')

    def get_all_symbols(self):
        return _get(f'{self.base_url}/beta/ref-data/symbols?token={self.token}')

    def get_all_options_dates(self):
        return _get(f'{self.base_url}/ref-data/options/symbols?token={self.token}')

    def get_earnings_per_share(self, symbol):
        data = _get(f'{self.base_url}/stable/stock/{symbol}/earnings/1/actualEPS?token={self.token}')
        if data:
            return data
