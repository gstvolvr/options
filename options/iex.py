import requests

class IEX:
    def __init__(self):
        self.token = None
        self.base_url = None

    def __get(self, url, json=True):
        try:
            response = requests.get(url)
            data = response.json() if json else response.text

        except:
            return None
        return data

    def get_quote(self, ticker):
        data = self.__get(f'{self.base_url}/stable/stock/{ticker}/batch?types=quote&token={self.token}')
        return data['quote']

    def get_industry(self, ticker):
        data = self.__get(f'{self.base_url}/stable/stock/{ticker}/company?token={self.token}')
        if data:
            return data['industry']

    def get_price(self, ticker, date=None):
        if date is None:
            data = self.__get(f'{self.base_url}/stable/stock/{ticker}/price?token={self.token}')
        else:
            price_list = self.__get(f'{self.base_url}/stable/stock/{ticker}/chart/1d/{date}?token={self.token}')
            data = sorted(price_list, key=lambda d: d['minute'])[-1]['marketClose']
        return data

    def get_quote(self, ticker):
        data = self.__get(f'{self.base_url}/stable/stock/{ticker}/quote?token={self.token}')
        if data:
            return data

    def has_options(self, ticker):
        dates = self.get_call_expiration_dates(ticker=ticker)

        if dates:
            return len(dates) > 0
        return False

    def get_last_dividend(self, ticker, time_frame='ytd'):
        data = self.__get(f'{self.base_url}/stable/stock/{ticker}/dividends/{time_frame}?token={self.token}')
        if data:
            return sorted(data, key=lambda d: d['exDate'])[-1]
        return {}

    def get_next_dividend(self, ticker):
        data = self.__get(f'{self.base_url}/stable/stock/{ticker}/dividends/next?token={self.token}')
        if data:
            return data
        return {}

    def get_call_expiration_dates(self, ticker):
        return self.__get(f'{self.base_url}/stable/stock/{ticker}/options?token={self.token}')

    def get_calls(self, ticker, expiration_date):
        return self.__get(f'{self.base_url}/stable/stock/{ticker}/options/{expiration_date}/call?token={self.token}')

    def get_symbols_csv(self):
        return self.__get(f'{self.base_url}/beta/ref-data/symbols?token={self.token}&format=csv', json=False)

    def get_all_options_dates(self):
        return self.__get(f'{self.base_url}/ref-data/options/symbols?token={self.token}')

    def get_earnings_per_share(self, ticker):
        data = self.__get(f'{self.base_url}/stable/stock/{ticker}/earnings/1/actualEPS?token={self.token}')
        if data:
            return data
