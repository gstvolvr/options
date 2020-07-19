from functools import partial
from typing import Dict
from dateutil.relativedelta import relativedelta
import datetime
import logging
import pandas as pd
import re
import time
import numpy as np


def get_returns(d_options, d_prices, d_dividends):
    assert 'symbol' in d_options.columns and 'symbol' in d_prices.columns and 'symbol' in d_dividends.columns

    d_options = d_options[(~d_options['is_adjusted']) & (d_options['ask'] != 0)]
    d_options['expiration_date'] = pd.to_datetime(d_options['expiration_date'], format='%Y%m%d')

    d_dividends = find_dividend(d_dividends, months=3)
    d_merged = d_options.merge(d_prices, left_on=['symbol', 'last_updated'], right_on=['symbol', 'previous_date'], how='left') \
                        .merge(d_dividends, on='symbol', how='left')
    d_merged['stock_price'] = d_merged['previous_stock_price']
    d_merged['stock_date'] = d_merged['previous_date']

    d_merged = d_merged[(d_merged['dividend_frequency'].notna()) &
                        (~d_merged['dividend_frequency'].isin(['final', 'unspecified'])) &
                        (d_merged['dividend_amount'].notna()) &
                        (d_merged['dividend_amount'] != '')]
    d_merged['mid'] = (d_merged['bid'] + d_merged['ask']) / 2
    d_merged['net'] = (d_merged['stock_price'] - d_merged['mid'])
    d_merged['premium'] = d_merged['strike_price'] - d_merged['net']
    d_merged['insurance'] = (d_merged['stock_price'] - d_merged['net']) / d_merged['stock_price']
    d_merged.drop(columns=['is_adjusted', 'open_interest', 'volume', 'type'], inplace=True)


    for col in d_merged.columns:
        if 'date' in col.lower() and col != 'expiration_date':
            d_merged[col] = pd.to_datetime(d_merged[col])

    for i in range(0, 4):
        d_merged[f'return_after_{i+1}_div'] = d_merged.apply(partial(days_to_next_event, i=i), axis=1)
    return d_merged

def find_dividend(d_dividends, months=3):
    for col in d_dividends.columns:
        if 'date' in col.lower():
            d_dividends[col] = pd.to_datetime(d_dividends[col])
    next_fields = [c for c in d_dividends.columns if c.startswith('next')]
    last_fields = [c for c in d_dividends.columns if c.startswith('last')]

    d_dividends['dividend_amount'] = d_dividends['next_dividend_amount'].combine_first(d_dividends['last_dividend_amount']).astype(np.float)
    d_dividends['dividend_frequency'] = d_dividends['next_dividend_frequency'].combine_first(d_dividends['last_dividend_frequency'])
    d_dividends['dividend_ex_date'] = d_dividends['next_dividend_ex_date'].combine_first(d_dividends['last_dividend_ex_date'] + pd.DateOffset(months=months))

    return d_dividends

def to_snake(name):
    return re.sub(f'(?<!^)(?=[A-Z])', '_', name).lower()

def days_to_first(row):
    today = datetime.datetime.today()
    return (row['dividend_ex_date'] - today).days

def days_to_next_event(row, i):
    frequency_map = {'quarterly': 4, 'semi-annual': 2, 'annual': 1, 'monthly': 12}
    days_to_first_dividend = days_to_first(row)
    months_in = 12 // frequency_map[row['dividend_frequency']]
    next_dividend_date = row['dividend_ex_date'] + relativedelta(months=months_in * (i+1))

    # if its Sunday, choose Monday
    if next_dividend_date.weekday() == 6:
        next_dividend_date += relativedelta(days=1)

    # if its Saturday, choose Monday
    elif next_dividend_date.weekday() == 5:
        next_dividend_date += relativedelta(days=2)

    next_event_date = min(row['expiration_date'], next_dividend_date)
    days_to_next_event = (next_event_date - datetime.datetime.today()).days + 2

    if days_to_next_event <= 0 or (next_dividend_date - row['expiration_date']).days >= months_in*30:
        return
    return ((row['dividend_amount'] * (i+1) + row['premium']) / row['net']) / days_to_next_event * 365


def get_dividends(iex, tickers):
    rows = []
    for ticker in tickers:
        row = {'symbol': ticker}
        next_dividend = iex.get_next_dividend(ticker)
        next_dividend_clean = {f'next_dividend_{to_snake(k)}': v for k, v in next_dividend.items()}

        last_dividend = iex.get_last_dividend(ticker)
        last_dividend_clean = {f'last_dividend_{to_snake(k)}': v for k, v in last_dividend.items()}

        row.update(next_dividend_clean)
        row.update(last_dividend_clean)

        rows.append(row)

    return pd.DataFrame(rows)

def get_eod_prices(iex, tickers):
    prices = []
    for i, ticker in enumerate(tickers):
        if (i != 0) and (i % 100) == 0:
            print(i, end=' ')
        quote = iex.get_quote(ticker)
        if quote['latestSource'] == 'Close':
            date = datetime.datetime.fromtimestamp(quote['closeTime'] / 1000)
            previous_date = date - datetime.timedelta(days=1)
            price = quote['close']
            prices.append({
                'symbol': ticker,
                'comany_name': quote['companyName'],
                'latest_stock_price': quote['close'],
                'latest_date': date.strftime('%Y-%m-%d'),
                'previous_stock_price': quote['previousClose'],
                'previous_date': previous_date.strftime('%Y-%m-%d')})
    d_prices = pd.DataFrame(prices)
    return d_prices

def get_eod_options(iex, tickers):
    """
    given a dictionary of tickers with dividends, get their expiration dates
    """

    data = []
    for i, ticker in enumerate(tickers):
        if (i != 0) and (i % 100) == 0:
            print(i, end=' ')
        # get list of expiration dates
        dates = iex.get_call_expiration_dates(ticker)

        # only look at dates in 2021
        dates = [d for d in dates if d[:4] == '2021']

        for expiration_date in dates:
            results = iex.get_calls(ticker, expiration_date)
            if results:
                for instance in results:
                    instance_snake = { to_snake(k): v for k, v in instance.items() }
                    data.append(instance_snake)
    return pd.DataFrame(data)

