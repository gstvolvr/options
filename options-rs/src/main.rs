use csv::Reader;
use std::env;
mod models;
mod services;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path: &String = &args[1];

    read(file_path);
}

fn read(file_path: &String) -> Result<(), Box<dyn std::error::Error>> {
		let mut rdr = Reader::from_path(file_path)?;

    for result in rdr.deserialize() {
        let record: models::company::Company = result?;
        println!("{:?}", record.company_name);
    }

    Ok(())

}

