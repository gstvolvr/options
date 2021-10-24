import options.iex
import os
__TEST_TICKER = 'BEN'

iex = options.iex.IEX()
iex.token = os.getenv('IEX_TOKEN')

dates = iex.get_call_expiration_dates(__TEST_TICKER)
results = iex.get_calls(__TEST_TICKER, dates[0])

# dict_keys(['ask', 'bid', 'cfiCode', 'close', 'closingPrice', 'contractDescription', 'contractName', 'contractSize', 'currency', 'exchangeCode', 'exchangeMIC', 'exerciseStyle', 'expirationDate', 'figi', 'high', 'isAdjusted', 'lastTradeDate', 'lastTradeTime', 'lastUpdated', 'low', 'marginPrice', 'open', 'openInterest', 'settlementPrice', 'side', 'strikePrice', 'symbol', 'type', 'volume', 'id', 'key', 'subkey', 'date', 'updated'])
for d in dates:
    calls = iex.get_calls(__TEST_TICKER, d)
    for call in calls:
        print(d, call['lastUpdated'],  call['closingPrice'], call['expirationDate'], call['strikePrice'])
