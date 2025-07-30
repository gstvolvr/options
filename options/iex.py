from datetime import datetime
import logging
import requests

REQUEST_DATE_FORMAT = '%Y%m%d'


def _get(url, json=True):
    """
    Sends a GET request to the specified URL and returns the response.

    Args:
        url (str): The URL to send the GET request to.
        json (bool): Whether to parse the response as JSON. Defaults to True.

    Returns:
        dict or str or None: The response from the server, parsed as JSON if specified,
                             otherwise as text. Returns an empty dict or None on error.
    """
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
    """
    Converts a date string from one format to another.

    Args:
        date (str): The date string to convert.
        current_date_format (str): The current format of the date string.

    Returns:
        str: The date string converted to the REQUEST_DATE_FORMAT.
    """
    return datetime.strptime(date, current_date_format).strftime(REQUEST_DATE_FORMAT)



class IEX:
    """A client for interacting with the IEX Cloud API."""
    def __init__(self):
        """Initializes the IEX client."""
        self.token = None
        self.base_url = 'https://cloud.iexapis.com'
        self.stats = {}

    def get_last_trading_day(self, offset=1):
       """
       Fetches the last trading day from IEX Cloud.

       Args:
           offset (int): The number of days to look back. Defaults to 1.

       Returns:
           str: The last trading day in 'YYYYMMDD' format.
       """
       data = _get(f'{self.base_url}/stable/ref-data/us/dates/trade/last/{offset}?token={self.token}').pop().get('date')
       return _convert_str_format(data, '%Y-%m-%d')

    def get_company(self, symbol):
        """
        Retrieves company information for a given stock symbol.

        Args:
            symbol (str): The stock symbol.

        Returns:
            dict: A dictionary containing company information.
        """
        data = _get(f'{self.base_url}/stable/stock/{symbol}/company?token={self.token}')
        return data or {}

    def get_quote(self, symbol):
        """
        Retrieves quote information for a given stock symbol.

        Args:
            symbol (str): The stock symbol.

        Returns:
            dict: A dictionary containing quote information.
        """
        data = _get(f'{self.base_url}/stable/stock/{symbol}/batch?types=quote&token={self.token}')
        return data['quote']

    def get_quote_from_last_trade_date(self, symbol):
        """
        Retrieves the closing price for a symbol on the last trading day.

        Args:
            symbol (str): The stock symbol.

        Returns:
            float or None: The closing price, or None if not available.
        """
        date_str = self.get_last_trading_day()
        return self.get_quote_from_date(symbol, date_str)

    def get_quote_from_date(self, symbol, date):
        """
        Retrieves the closing price for a symbol on a specific date.

        Args:
            symbol (str): The stock symbol.
            date (str): The date in 'YYYYMMDD' format.

        Returns:
            float or None: The closing price, or None if not available.
        """
        data = _get(f'{self.base_url}/stable/stock/{symbol}/chart/date/{date}?chartByDay=true&types=quote&token={self.token}')
        if data:
          return data.pop().get('close')
        else:
          return None

    def get_price(self, symbol, date=None):
        """
        Retrieves the price of a stock, either the latest or for a specific date.

        Args:
            symbol (str): The stock symbol.
            date (str, optional): The date in 'YYYYMMDD' format. Defaults to None.

        Returns:
            float or None: The price of the stock.
        """
        if date is None:
            data = _get(f'{self.base_url}/stable/stock/{symbol}/price?token={self.token}')
        else:
            price_list = _get(f'{self.base_url}/stable/stock/{symbol}/chart/1d/{date}?token={self.token}')
            data = sorted(price_list, key=lambda d: d['minute'])[-1]['marketClose']
        return data

    def get_stats(self, symbol):
        """
        Retrieves key stats for a given stock symbol.

        Args:
            symbol (str): The stock symbol.

        Returns:
            dict: A dictionary containing key stats.
        """
        if symbol in self.stats:
            return self.stats[symbol]
        else:
            self.stats[symbol] = _get(f'{self.base_url}/stable/stock/{symbol}/stats?token={self.token}') or {}
        return self.stats[symbol]

    def get_dividend_yield(self, symbol):
        """
        Retrieves the dividend yield for a given stock symbol.

        Args:
            symbol (str): The stock symbol.

        Returns:
            float or None: The dividend yield, or None if not available.
        """
        symbol_stats = self.get_stats(symbol)
        if 'dividendYield' in symbol_stats:
            return symbol_stats['dividendYield']
        return None

    def has_options(self, symbol):
        """
        Checks if a stock has options available.

        Args:
            symbol (str): The stock symbol.

        Returns:
            bool: True if options are available, False otherwise.
        """
        dates = self.get_call_expiration_dates(symbol=symbol)
        return dates is not None and len(dates) > 0

    def has_dividends(self, symbol):
        """
        Checks if a stock has dividends.

        Args:
            symbol (str): The stock symbol.

        Returns:
            bool: True if the stock has dividends, False otherwise.
        """
        symbol_stats = self.get_stats(symbol)
        return ('nextDividendDate' in symbol_stats and symbol_stats['nextDividendDate'] is not None) or \
               ('exDividendDate' in symbol_stats and symbol_stats['exDividendDate'] is not None)

    def get_last_dividend(self, symbol, time_frame='1y'):
        """
        Retrieves the last dividend paid for a stock.

        Args:
            symbol (str): The stock symbol.
            time_frame (str): The time frame to look back for dividends. Defaults to '1y'.

        Returns:
            dict: A dictionary containing information about the last dividend.
        """
        data = _get(f'{self.base_url}/stable/stock/{symbol}/dividends/{time_frame}?token={self.token}')
        if data:
            return sorted(data, key=lambda d: d['exDate'])[-1]
        return {}

    def get_next_dividend(self, symbol):
        """
        Retrieves the next dividend to be paid for a stock.

        Args:
            symbol (str): The stock symbol.

        Returns:
            dict: A dictionary containing information about the next dividend.
        """
        data = _get(f'{self.base_url}/stable/stock/{symbol}/dividends/next?token={self.token}')
        if data:
            return data[0]
        return {}

    def get_call_expiration_dates(self, symbol):
        """
        Retrieves the available expiration dates for call options.

        Args:
            symbol (str): The stock symbol.

        Returns:
            list or None: A list of expiration dates, or None if not available.
        """
        return _get(f'{self.base_url}/stable/stock/{symbol}/options?token={self.token}')

    def get_calls(self, symbol, expiration_date):
        """
        Retrieves call option data for a specific expiration date.

        Args:
            symbol (str): The stock symbol.
            expiration_date (str): The expiration date in 'YYYYMMDD' format.

        Returns:
            list or None: A list of call options, or None if not available.
        """
        return _get(f'{self.base_url}/stable/stock/{symbol}/options/{expiration_date}/call?token={self.token}')

    def get_all_symbols(self):
        """Retrieves all stock symbols from IEX Cloud."""
        return _get(f'{self.base_url}/beta/ref-data/symbols?token={self.token}')

    def get_all_options_dates(self):
        """Retrieves all available option expiration dates."""
        return _get(f'{self.base_url}/ref-data/options/symbols?token={self.token}')

    def get_earnings_per_share(self, symbol):
        """
        Retrieves the actual earnings per share for a stock.

        Args:
            symbol (str): The stock symbol.

        Returns:
            dict or None: A dictionary containing EPS data, or None if not available.
        """
        data = _get(f'{self.base_url}/stable/stock/{symbol}/earnings/1/actualEPS?token={self.token}')
        if data:
            return data
