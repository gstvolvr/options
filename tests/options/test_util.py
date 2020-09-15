import unittest
import datetime
import options.util as under_test
from unittest.mock import patch


@patch('options.util.get_datetime_today')
class UtilTests(unittest.TestCase):

    def test__get_previous_trading_date_thursday(self, get_datetime_today):
        get_datetime_today.return_value = datetime.datetime(2020, 9, 10)
        self.assertEqual('2020-09-09', under_test.get_previous_trading_date())

    def test__get_previous_trading_date_friday(self, get_datetime_today):
        get_datetime_today.return_value = datetime.datetime(2020, 9, 11)
        self.assertEqual('2020-09-10', under_test.get_previous_trading_date())

    def test__get_previous_trading_date_saturday(self, get_datetime_today):
        get_datetime_today.return_value = datetime.datetime(2020, 9, 12)
        self.assertEqual('2020-09-10', under_test.get_previous_trading_date())

    def test__get_previous_trading_date_sunday(self, get_datetime_today):
        get_datetime_today.return_value = datetime.datetime(2020, 9, 13)
        self.assertEqual('2020-09-10', under_test.get_previous_trading_date())

    def test__get_previous_trading_date_monday(self, get_datetime_today):
        get_datetime_today.return_value = datetime.datetime(2020, 9, 14)
        self.assertEqual('2020-09-11', under_test.get_previous_trading_date())
