from dateutil.relativedelta import relativedelta
import datetime
import re

FREQUENCY_MAPPING = {'quarterly': 3, 'semi-annual': 6, 'annual': 12, 'monthly': 1, 'bimonthly': 2}

MONTHS_IN_QUARTER = 3


def to_snake(name):
    return re.sub(f'(?<!^)(?=[A-Z])', '_', name).lower()


def days_to_first(row):
    today = datetime.datetime.today()
    return (row['dividend_ex_date'] - today).days


def calculate_return_after_dividends(row, n_dividends):
    next_dividend_date = row['dividend_ex_date'] + relativedelta(months=MONTHS_IN_QUARTER * (n_dividends+1))

    # if its Sunday, choose Monday
    if next_dividend_date.weekday() == 6:
        next_dividend_date += relativedelta(days=1)

    # if its Saturday, choose Monday
    elif next_dividend_date.weekday() == 5:
        next_dividend_date += relativedelta(days=2)

    next_event_date = min(row['expiration_date'], next_dividend_date)
    days_to_next_event = (next_event_date - datetime.datetime.today()).days + 2

    if days_to_next_event <= 0 or (next_dividend_date - row['expiration_date']).days >= MONTHS_IN_QUARTER*30:
        return
    return ((float(row['dividend_quarterly_amount']) * (n_dividends+1) + float(row['premium'])) / float(row['net'])) / days_to_next_event * 365


def get_datetime_today():
    """splitting this out to make testing easier"""
    return datetime.datetime.today()


def get_previous_trading_date():
    """
    get the last trading date, unless its the weekend, in which case we want to get last Thursday
    """

    today = get_datetime_today()
    weekend = today.weekday() in [5, 6]

    if today.weekday() == 0:
        delta = 3
    elif weekend:
        delta = today.weekday() - 3
    else:
        delta = 1

    previous_trading_day = today - relativedelta(days=delta)
    previous_trading_day_fmt = datetime.datetime.strftime(previous_trading_day, '%Y-%m-%d')
    return previous_trading_day_fmt
