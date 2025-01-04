use std::{
    error::Error,
    fs::File,
    process,
    collections::HashSet,
    io::SeekFrom,
    io::Seek,
    io::Read,
    io::BufReader
};
use clap::{Parser};

/// Simple program to parse computed address against used addresses
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File containing a list of computed addresses
    #[arg(short, long)]
    computed_addresses: String,

    /// File containing a list of used addresses
    #[arg(short, long)]
    used_addresses: String,

    /// Offset in the file containing the list of computed addresses to start on
    #[arg(short, long, default_value_t = 0)]
    offset: u64,

    /// Address format
    #[arg(value_enum)]
    address_format: common::AddressFormat,
}


#[derive(Debug, serde::Deserialize)]
struct AccountBalance {
    account: String,
    balance: f64,
}

fn load(used_path: &String, used_addresses: &mut HashSet<String>) -> Result<(), Box<dyn Error>> {
    let file = File::open(used_path)?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b'\t')
        .double_quote(false)
        .from_reader(file);

    for result in rdr.deserialize() {
        let record: AccountBalance = result?;
        used_addresses.insert(record.account);
    }
    Ok(())
}


fn load_json(used_path: &String, used_addresses: &mut HashSet<String>) -> Result<(), Box<dyn Error>> {
    // Read the file content
    let mut file = BufReader::new(File::open(used_path).expect("Failed to open the file"));
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).expect("Failed to read the file");

    // Parse the JSON content
    let records: Vec<AccountBalance> =
        serde_json::from_str(&file_content).expect("Failed to parse JSON");

    for result in records {
        used_addresses.insert(result.account);
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let mut used_addresses = HashSet::new();

    if args.address_format == common::AddressFormat::XRP {
        if let Err(err) = load_json(&args.used_addresses, &mut used_addresses) {
            println!("{}", err);
            process::exit(1);
        }    
    } else {
        if let Err(err) = load(&args.used_addresses, &mut used_addresses) {
            println!("{}", err);
            process::exit(1);
        }
    }
    println!("Used addresses loaded: {}",used_addresses.len());

    let file_path = args.computed_addresses;
 
    let mut file = BufReader::new(File::open(file_path)?);
    file.seek(SeekFrom::Start(args.offset))?;
    let size: usize;

    match args.address_format {
        common::AddressFormat::ETH => {
            size = common::ETH_ADDRESS_SIZE;
        }
        common::AddressFormat::TRX => {
            size = common::TRX_ADDRESS_SIZE;
        }
        common::AddressFormat::DOGE => {
            size = common::DOGE_ADDRESS_SIZE;
        }
        common::AddressFormat::ZEC => {
            size = common::ZEC_ADDRESS_SIZE;
        }
        common::AddressFormat::LTC => {
            size = common::LTC_ADDRESS_SIZE;
        }    
        common::AddressFormat::XRP => {
            size = common::XRP_ADDRESS_SIZE;
        }
        common::AddressFormat::BTC44 => {
            size = common::BTC44_ADDRESS_SIZE;
        }
        common::AddressFormat::BTC49 => {
            size = common::BTC49_ADDRESS_SIZE;
        }
    }

    let mut buffer = vec![0;size];//::with_capacity(size);//: vec![0;size];

    loop {
        file.read_exact(&mut buffer)?;
        let mut address;
        match args.address_format {
            common::AddressFormat::ETH => {
                address = hex::encode(&buffer);
                //address.insert(0, 'x');
                //address.insert(0, '0');
            }
            common::AddressFormat::TRX | common::AddressFormat::DOGE | common::AddressFormat::ZEC | common::AddressFormat::LTC | common::AddressFormat::BTC44 | common::AddressFormat::BTC49 => {
                address = String::from_utf8(buffer[0..size-1].to_vec()/* .clone()*/).unwrap();
            }    
            common::AddressFormat::XRP => {
                address = String::from_utf8(buffer[0..size-1].to_vec()/* .clone()*/).unwrap();
                address.replace_range(0..1,"r");
                //address = 'r'+ address.strip_prefix('1'').unwrap();
            }
        }
    
        match used_addresses.get(&address) {
            Some(balance) => {
                let seed = (file.stream_position()? - size as u64)/ size as u64;
                println!("{address}: {balance}: {seed}");
            },
            None => ()
        }
    }
}
