# Deep in the money covered calls

Buy-write screener for deep in the money covered call strategy. Idea is to find
positions, with expiration dates usually over a year in the future, that provide a good
balance of downward protection (15% to 30%) and expected return (premium + dividends).

Positions are loaded into
[this](https://docs.google.com/spreadsheets/d/1dhLDNkZbI2-7Fm4jXRreL-S6oTRExumvJRH1fEQIiOs/edit?usp=sharing) Google Sheet and are updated every 15 minutes on trading days.

# Data
I pull data from free 2 sources:
1. IEX Cloud (dividend information)
2. TDAmeritrade API (everything else)

I track the following attributes:
* name	
* industry	
* stock price
* net price of position 
* call strike price
* call expiration date
* insurance	
* premium	
* dividend	
* ex dividend date	
* expected return after 1 dividend
* expected return after 2 dividends	
* expected return after 3 dividends	
* expected return after 4 dividends	
* latest bid	
* latest mid point	
* latest ask
