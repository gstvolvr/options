CREATE TABLE universe.symbols(
    symbol VARCHAR(6) PRIMARY KEY,
    in_universe BOOLEAN DEFAULT FALSE,
    date DATE,
    db_updated TIMESTAMP DEFAULT NOW()
);

CREATE TABLE universe.companies(
  symbol VARCHAR(6) REFERENCES universe.symbols(symbol) PRIMARY KEY,
  company_name VARCHAR(100),
  employees INTEGER,
  exchange VARCHAR(100),
  industry VARCHAR(100),
  description TEXT,
  sector VARCHAR(100),
  issue_type VARCHAR(4),
  tags TEXT[],
  state VARCHAR(50),
  country VARCHAR(2),
  db_updated TIMESTAMP DEFAULT NOW()
);

CREATE TABLE universe.eod_prices(
  symbol VARCHAR(6) REFERENCES universe.companies(symbol) PRIMARY KEY,
  latest_stock_price FLOAT,
  latest_date DATE,
  previous_stock_price FLOAT,
  previous_date DATE,
  db_updated TIMESTAMP DEFAULT NOW()
);

CREATE TABLE universe.eod_call_options(
  symbol VARCHAR(5) REFERENCES universe.companies(symbol),
  id VARCHAR(50) PRIMARY KEY,
  expiration_date DATE,
  contract_size INTEGER,
  strike_price FLOAT,
  closing_price FLOAT,
  last_updated DATE,
  type VARCHAR(20),
  volume INTEGER,
  bid FLOAT,
  ask FLOAT,
  is_adjusted BOOLEAN,
  db_updated TIMESTAMP DEFAULT NOW()
);

CREATE TABLE universe.returns(
  symbol VARCHAR(6) REFERENCES universe.companies(symbol),
  id VARCHAR(50) REFERENCES universe.eod_call_options(id) PRIMARY KEY,
  net FLOAT,
  premium FLOAT,
  insurance FLOAT,
  return_after_1_div FLOAT,
  return_after_2_div FLOAT,
  return_after_3_div FLOAT,
  db_updated TIMESTAMP DEFAULT NOW()
);


CREATE TABLE universe.dividends(
  symbol VARCHAR(6) REFERENCES universe.companies(symbol) PRIMARY KEY,
  ex_date DATE,
  payment_date DATE,
  record_date DATE,
  declared_date DATE,
  amount FLOAT,
  currency VARCHAR(5),
  description VARCHAR(100),
  frequency VARCHAR(20),
  calculated BOOLEAN,
  db_updated TIMESTAMP DEFAULT NOW()
);

