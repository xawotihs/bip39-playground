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

        match args.address_format {
            common::AddressFormat::ETH => {
                let mut formatted_address: String = "0x".to_owned();
                formatted_address.push_str(&record.address);
                scraper::scrape_debank(&formatted_address).await?;
            }
            common::AddressFormat::DOGE => {
                let mut formatted_address: String = "".to_owned();
                formatted_address.push_str(&record.address);
                scraper::scrape_doge_explorer(&formatted_address).await?;
            }
            common::AddressFormat::BTC44 | common::AddressFormat::BTC49 => {
                let mut formatted_address: String = "".to_owned();
                formatted_address.push_str(&record.address);
                scraper::scrape_bitcoin_explorer(&formatted_address).await?;
            }
            _ => todo!()
        }
    }

    Ok(())
}