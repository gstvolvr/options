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



class IEX:
    def __init__(self):
        self.token = None
        self.base_url = 'https://cloud.iexapis.com'
        self.stats = {}

    def get_last_trading_day(self, offset=1):
       data = _get(f'{self.base_url}/stable/ref-data/us/dates/trade/last/{offset}?token={self.token}').pop().get('date')
       return _convert_str_format(data, '%Y-%m-%d')

    def get_company(self, symbol):
        data = _get(f'{self.base_url}/stable/stock/{symbol}/company?token={self.token}')
        return data or {}

    def get_quote(self, symbol):
        data = _get(f'{self.base_url}/stable/stock/{symbol}/batch?types=quote&token={self.token}')
        return data['quote']

    def get_quote_from_last_trade_date(self, symbol):
        date_str = self.get_last_trading_day().pop().get('date')
        return self.get_quote_from_date(symbol, date_str)

    def get_quote_from_date(self, symbol, date):
        data = _get(f'{self.base_url}/stable/stock/{symbol}/chart/date/{date}?chartByDay=true&types=quote&token={self.token}')
        if data:
          return data.pop().get('close')
        else:
          return None

    def get_price(self, symbol, date=None):
        if date is None:
            data = _get(f'{self.base_url}/stable/stock/{symbol}/price?token={self.token}')
        else:
            price_list = _get(f'{self.base_url}/stable/stock/{symbol}/chart/1d/{date}?token={self.token}')
            data = sorted(price_list, key=lambda d: d['minute'])[-1]['marketClose']
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
