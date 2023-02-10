from datetime import datetime
import logging
import requests
import time


REQUEST_DATE_FORMAT = '%Y%m%d'
# can only make 120 requests / minute
SLEEP_SECONDS = 0.51


def _get(url, sleep: bool, json: bool = True):
    try:
        response = requests.get(url)
        data = response.json() if json else response.text
    except Exception as e:
        logging.debug(f'ERROR: {e}')
        if json:
            data = {}
        else:
            data = None

    if sleep:
        time.sleep(SLEEP_SECONDS)
    return data


def _convert_str_format(date, current_date_format):
    return datetime.strptime(date, current_date_format).strftime(REQUEST_DATE_FORMAT)


class TDAmeritrade:
    def __init__(self):
        self.token = None
        self.base_url = 'https://api.tdameritrade.com/v1'

    def get_chains(self, ticker: str, from_date: str, sleep: bool = True):
        return _get(f'{self.base_url}/marketdata/chains?apikey={self.token}&symbol={ticker}&contractType=CALL&includeQuotes=TRUE&optionType=S&fromDate={from_date}', sleep=sleep)

    def get_quote(self, ticker: str, sleep: bool = True):
        """
        ETF:
        {
          "symbol": "string", "description": "string",
          "bidPrice": 0,
          "bidSize": 0,
          "bidId": "string",
          "askPrice": 0,
          "askSize": 0,
          "askId": "string",
          "lastPrice": 0,
          "lastSize": 0,
          "lastId": "string",
          "openPrice": 0,
          "highPrice": 0,
          "lowPrice": 0,
          "closePrice": 0,
          "netChange": 0,
          "totalVolume": 0,
          "quoteTimeInLong": 0,
          "tradeTimeInLong": 0,
          "mark": 0,
          "exchange": "string",
          "exchangeName": "string",
          "marginable": false,
          "shortable": false,
          "volatility": 0,
          "digits": 0,
          "52WkHigh": 0,
          "52WkLow": 0,
          "peRatio": 0,
          "divAmount": 0,
          "divYield": 0,
          "divDate": "string",
          "securityStatus": "string",
          "regularMarketLastPrice": 0,
          "regularMarketLastSize": 0,
          "regularMarketNetChange": 0,
          "regularMarketTradeTimeInLong": 0
        }

        Equity:
        {
          "symbol": "string",
          "description": "string",
          "bidPrice": 0,
          "bidSize": 0,
          "bidId": "string",
          "askPrice": 0,
          "askSize": 0,
          "askId": "string",
          "lastPrice": 0,
          "lastSize": 0,
          "lastId": "string",
          "openPrice": 0,
          "highPrice": 0,
          "lowPrice": 0,
          "closePrice": 0,
          "netChange": 0,
          "totalVolume": 0,
          "quoteTimeInLong": 0,
          "tradeTimeInLong": 0,
          "mark": 0,
          "exchange": "string",
          "exchangeName": "string",
          "marginable": false,
          "shortable": false,
          "volatility": 0,
          "digits": 0,
          "52WkHigh": 0,
          "52WkLow": 0,
          "peRatio": 0,
          "divAmount": 0,
          "divYield": 0,
          "divDate": "string",
          "securityStatus": "string",
          "regularMarketLastPrice": 0,
          "regularMarketLastSize": 0,
          "regularMarketNetChange": 0,
          "regularMarketTradeTimeInLong": 0
        }
        """
        return _get(f'{self.base_url}/marketdata/quotes?apikey={self.token}&symbol={ticker}', sleep=sleep)
