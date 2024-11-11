import logging
import requests
from urllib.parse import urlencode, urlparse, parse_qs

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


class Schwab:
    def __init__(self):
        self.token = None
        self.base_url = 'https://api.schwabapi.com'
        self.redirect_uri = 'https://developer.schwab.com/oauth2-redirect.html'
        self.auth_url = f'{self.base_url}/v1/oauth/authorize'
        self.token_url = f'{self.base_url}/v1/oauth/token'
        self.stats = {}
        self.token = None

    def get_auth_code(self, client_id):
        auth_url = self.get_auth_url(client_id)
        print(auth_url)
        params = {
            'response_type': 'code',
            'client_id': client_id,
            'redirect_uri': self.redirect_uri,
            'scope': 'readonly',
            # 'state': ,
        }
        url = f"{self.auth_url}?{urlencode(params)}"
        print(f"Please go to the following URL and authorize the application:\n{url}\n")

        # Step 2: User authorizes and gets redirected to the redirect_uri with a code
        redirect_response = input("Paste the full redirect URL here: ")
        parsed_url = urlparse(redirect_response)
        auth_code = parse_qs(parsed_url.query).get('code')[0]
        return auth_code
        # response = requests.post(
        #     auth_url,
        #     data=params,
        #     auth=(client_id, client_secret),
        # )
        # print(response.json())
        # # self.token = response.json()["access_token"]

    def get_auth_url(self, client_id):
        return f"{self.base_url}/v1/oauth/authorize?response_type=code&client_id={client_id}&scope=readonly&redirect_uri={self.redirect_uri}"

    def auth(self):
        pass

    # def get_last_trading_day(self, offset=1):
    #    data = _get(f'{self.base_url}/stable/ref-data/us/dates/trade/last/{offset}?token={self.token}').pop().get('date')
    #    return _convert_str_format(data, '%Y-%m-%d')

    # def get_company(self, symbol):
    #     data = _get(f'{self.base_url}/stable/stock/{symbol}/company?token={self.token}')
    #     return data or {}

    def get_quote(self, symbol):
        data = _get(f'{self.base_url}/marketdata/v1?quotes?symbols={symbol}&fields=quote&indicative=false')
        return data

    # def get_quote_from_last_trade_date(self, symbol):
    #     date_str = self.get_last_trading_day().pop().get('date')
    #     return self.get_quote_from_date(symbol, date_str)
    #
    # def get_quote_from_date(self, symbol, date):
    #     data = _get(f'{self.base_url}/stable/stock/{symbol}/chart/date/{date}?chartByDay=true&types=quote&token={self.token}')
    #     if data:
    #       return data.pop().get('close')
    #     else:
    #       return None
    #
    # def get_price(self, symbol, date=None):
    #     if date is None:
    #         data = _get(f'{self.base_url}/stable/stock/{symbol}/price?token={self.token}')
    #     else:
    #         price_list = _get(f'{self.base_url}/stable/stock/{symbol}/chart/1d/{date}?token={self.token}')
    #         data = sorted(price_list, key=lambda d: d['minute'])[-1]['marketClose']
    #     return data
    #
    # def get_stats(self, symbol):
    #     if symbol in self.stats:
    #         return self.stats[symbol]
    #     else:
    #         self.stats = {symbol: _get(f'{self.base_url}/stable/stock/{symbol}/stats?token={self.token}') or {}}
    #     return self.stats[symbol]
    #
    # def get_dividend_yield(self, symbol):
    #     symbol_stats = self.get_stats(symbol)
    #     if 'dividendYield' in symbol_stats:
    #         return symbol_stats['dividendYield']
    #     return None
    #
    # def has_options(self, symbol):
    #     dates = self.get_call_expiration_dates(symbol=symbol)
    #     return dates is not None and len(dates) > 0
    #
    # def has_dividends(self, symbol):
    #     symbol_stats = self.get_stats(symbol)
    #     return ('nextDividendDate' in symbol_stats and symbol_stats['nextDividendDate'] is not None) or \
    #            ('exDividendDate' in symbol_stats and symbol_stats['exDividendDate'] is not None)
    #
    # def get_last_dividend(self, symbol, time_frame='1y'):
    #     data = _get(f'{self.base_url}/stable/stock/{symbol}/dividends/{time_frame}?token={self.token}')
    #     if data:
    #         return sorted(data, key=lambda d: d['exDate'])[-1]
    #     return {}
    #
    # def get_next_dividend(self, symbol):
    #     data = _get(f'{self.base_url}/stable/stock/{symbol}/dividends/next?token={self.token}')
    #     if data:
    #         return data[0]
    #     return {}
    #
    # def get_call_expiration_dates(self, symbol):
    #     return _get(f'{self.base_url}/stable/stock/{symbol}/options?token={self.token}')
    #
    # def get_calls(self, symbol, expiration_date):
    #     return _get(f'{self.base_url}/stable/stock/{symbol}/options/{expiration_date}/call?token={self.token}')
    #
    # def get_all_symbols(self):
    #     return _get(f'{self.base_url}/beta/ref-data/symbols?token={self.token}')
    #
    # def get_all_options_dates(self):
    #     return _get(f'{self.base_url}/ref-data/options/symbols?token={self.token}')
    #
    # def get_earnings_per_share(self, symbol):
    #     data = _get(f'{self.base_url}/stable/stock/{symbol}/earnings/1/actualEPS?token={self.token}')
    #     if data:
    #        return data
