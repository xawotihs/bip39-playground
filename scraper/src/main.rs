use std::error::Error;
use clap::Parser;
use std::fs::File;

/// Simple program to parse computed address against used addresses
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File containing a list of addresses to check
    #[arg(short, long)]
    addresses: String,

    /// Address format
    #[arg(value_enum)]
    address_format: common::AddressFormat
}

#[derive(Debug, serde::Deserialize)]
struct Record {
    address: String,
    address2: String,
    balance: f64
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let file = File::open(args.addresses)?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b' ')
        .double_quote(false)
        .from_reader(file);

    for result in rdr.deserialize() {
        let record: Record = result?;
        let mut formatted_address: String = "0x".to_owned();
        formatted_address.push_str(&record.address);
        scraper::scrape_debank(&formatted_address).await?;
    }

    Ok(())
}