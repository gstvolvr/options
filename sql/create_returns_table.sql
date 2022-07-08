DROP TABLE IF EXISTS universe.returns;
CREATE TABLE universe.returns(
    symbol VARCHAR(10),
    company_name VARCHAR(100),
    industry VARCHAR(200),
    last FLOAT,
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
    return_after_4_div FLOAT,
    bid FLOAT,
    mid FLOAT,
    ask FLOAT,
    quote_date DATE,

    UNIQUE(symbol, strike_price, expiration_date)
)
