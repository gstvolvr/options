from options import iex, util
import pandas as pd

"""
one off testing script
"""

with open('symbols.csv') as r:
    symbols = [s for s in r.read().split('\n') if s != '']

iex = iex.IEX()
iex.token = os.getenv('IEX_TOKEN')

d_dividends = util.get_dividends(iex, symbols)
d_dividends = d_dividends[(d_dividends['next_dividend_amount'].notna() & d_dividends['next_dividend_amount'] != '') | (d_dividends['last_dividend_amount'].notna() & d_dividends['last_dividend_amount'] != '')]

d_prices = util.get_eod_prices(iex, symbols)
d_options = util.get_eod_options(iex, symbols)
d_industries = util.get_industries(iex, symbols)
d_prices = util.get_eod_prices(iex, symbols)

d_industries = util.get_industries(iex, symbols)
d_options = util.get_eod_options(iex, symbols)

d_returns = util.get_returns(d_options, d_prices, d_dividends).merge(d_industries, on=['symbol'])

cols = [
   'symbol',
   'comany_name',
   'industry',
   'stock_price',
   'net',
   'strike_price',
   'expiration_date',
   'insurance',
   'premium',
   'dividend_amount',
   'dividend_ex_date',
   'return_after_1_div',
   'return_after_2_div',
   'return_after_3_div',
   'bid',
   'mid',
   'ask',
   'previous_date']

d_returns[cols] \
   .sort_values(by=['symbol', 'expiration_date', 'strike_price']) \
   .to_csv('returns.csv', index=False)
