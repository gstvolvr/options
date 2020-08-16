create table universe.symbols(
  symbol VARCHAR(5),
  db_updated TIMESTAMP DEFAULT NOW()
)

CREATE TABLE universe.options(
  symbol VARCHAR(5),
  comany_name VARCHAR(100),
  industry VARCHAR(100),
  stock_price FLOAT,
  net FLOAT,
  strike_price FLOAT,
  expiration_date DATE,
  insurance FLOAT,
  premium FLOAT,
  dividend_amount FLOAT,
  dividend_ex_date DATE,
  return_after_1_div FLOAT,
  return_after_2_div FLOAT,
  return_after_3_div FLOAT,
  bid FLOAT,
  mid FLOAT,
  ask FLOAT,
  previous_date DATE,
  db_updated TIMESTAMP DEFAULT NOW()
);

CREATE TABLE universe.dividends(
  symbol VARCHAR(5),
  last_dividend_ex_date DATE,
  last_dividend_payment_date DATE,
  last_dividend_record_date DATE,
  last_dividend_declared_date DATE,
  last_dividend_amount FLOAT,
  last_dividend_flag BOOLEAN,
  last_dividend_currency VARCHAR(3),
  last_dividend_description VARCHAR(100),
  last_dividend_frequency VARCHAR(20),
  last_dividend_date DATE,
  next_dividend_ex_date DATE,
  next_dividend_payment_date DATE,
  next_dividend_record_date DATE,
  next_dividend_declared_date DATE,
  next_dividend_amount FLOAT,
  next_dividend_flag BOOLEAN,
  next_dividend_currency VARCHAR(3),
  next_dividend_description VARCHAR(100),
  next_dividend_frequency VARCHAR(20),
  dividend_amount FLOAT,
  db_updated TIMESTAMP DEFAULT NOW()
);

CREATE TABLE universe.prices(
  symbol VARCHAR(5),
  latest_stock_price FLOAT,
  latest_date DATE,
  previous_stock_price FLOAT,
  previous_date DATE,
  db_updated TIMESTAMP DEFAULT NOW()
);

CREATE TABLE universe.companies(
  symbol VARCHAR(5),
  name VARCHAR(100),
  industry VARCHAR(100),
  db_updated TIMESTAMP DEFAULT NOW()
);

