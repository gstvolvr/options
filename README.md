# Deep in the money covered calls

Buy-write screener for deep in the money covered call strategy. Idea is to find
positions, with expiration dates usually over a year in the future, that provide a good
balance of downward protection (15% to 30%) and expected return (premium + dividends).

I discuss the strategy in more detail [here](https://www.oliver.dev/posts/2025/05/options-screener-part-i.html)

Positions are loaded into
[this](https://docs.google.com/spreadsheets/d/1dhLDNkZbI2-7Fm4jXRreL-S6oTRExumvJRH1fEQIiOs/edit?usp=sharing) Google Sheet and are updated every 15 minutes on trading days.

# Data
I pull data from the Schwab API and track the following attributes:
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
