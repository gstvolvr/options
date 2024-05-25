use csv::Reader;
use std::env;
use chrono::{DateTime, Utc, NaiveDate, Duration};
pub mod models;


fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];
    update_returns();
}

fn _process(record: models::options::Options) {
    let mid = (record.bid - record.ask) / 2.0;
    let net = record.last - mid;
    let premium = record.strike_price - net;
    let insurance = (record.last - net) / row.last;

    if premium < 0.05 {
        return
    }

    //let returns: models::returns::Returns;

    for j in 0..6 {
        util.calculate_return_after_dividends(record, n_dividends=j)
    }
}
//def _process(r):
//    record = r.copy()
//    record['mid'] = (float(row['bid']) + float(row['ask'])) / 2
//    record['net'] = (float(row['last']) - float(row['mid']))
//    record['premium'] = float(row['strike_price']) - float(row['net'])
//    record['insurance'] = (float(row['last']) - float(row['net'])) / float(row['last'])
//
//    # ignore unrealistic premiums
//    if record['premium'] < 0.05:
//        return None
//
//    for j in range(0, 6):
//        record[f'return_after_{j+1}_div'] = util.calculate_return_after_dividends(row, n_dividends=j)
//    record['dividend_ex_date'] = datetime.datetime.strftime(row['dividend_ex_date'], '%Y-%m-%d')
//    record['expiration_date'] = datetime.datetime.strftime(row['expiration_date'], '%Y-%m-%d')
//    return record

fn update_returns() -> Result<(), Box<dyn std::error::Error>> {
    let mut rdr = Reader::from_path("../data/options.csv")?;

    for result in rdr.deserialize() {
        let record: models::options::Options = result?;
        let expiration_date: i64 = record.expiration_date / 1000;
        // TODO: add proper handling in case this isn't a valid value
        let expiration_date: NaiveDate = DateTime::from_timestamp(expiration_date, 0).unwrap().date_naive();
        println!("{:?}", expiration_date);

        let today = Utc::now().date_naive();
        println!("{:?}", today);

        // we only want to process things in the next 20 or so months
        if expiration_date > today + Duration::days(30*20) {
            println!("{:?}", "far out man");
            continue
        }

        if record.last * 0.50 > record.strike_price {
            println!("The strike price is too high compared to last: {:?} vs {:?}", record.strike_price, record.last);
            continue
        }

        let returns = _process(record);
        //returns = _process(record)
        //if returns is not None and returns['return_after_1_div'] is not None:
        //    if writer is None:
        //        writer = csv.DictWriter(w, fieldnames=returns.keys())
        //        writer.writeheader()
        //    writer.writerecord(returns)
    }

    Ok(())
}

