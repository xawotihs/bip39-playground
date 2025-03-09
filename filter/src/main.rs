use std::{
    error::Error,
    fs::File,
    process,
    collections::HashSet,
    io::SeekFrom,
    io::Seek,
    io::Read,
    io::BufReader,
    io::prelude::*,
    io::self
};
use clap::{Parser};
use std::sync::{Arc, Mutex};
use std::thread;
use crossbeam::channel::{bounded, Sender, Receiver};
use std::cmp;
use num_cpus;
use sysinfo::{System};

/// Simple program to parse computed address against used addresses
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File containing a list of computed addresses or STDIN if empty
    #[arg(short, long, default_value = "")]
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

#[derive(Debug, serde::Deserialize)]
struct AddressBalance {
    address: String,
    balance: f64,
}

fn load_accounts(used_path: &String, used_addresses: &mut HashSet<String>) -> Result<(), Box<dyn Error>> {
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

fn load_addresses(used_path: &String, used_addresses: &mut HashSet<String>, has_header: bool, delimiter: u8) -> Result<(), Box<dyn Error>> {
    let file = File::open(used_path)?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(has_header)
        .delimiter( delimiter)
        .double_quote(false)
        .from_reader(file);

    for result in rdr.deserialize() {
        let record: AddressBalance = result?;
        used_addresses.insert(record.address);
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
    let mut used_addresses = Arc::new(load_set(&args.used_addresses,args.address_format)?);//HashSet::new();
    let num_threads = cmp::max(1, num_cpus::get());
    let mut sys = System::new_all();
    sys.refresh_memory();
    let free_memory = sys.available_memory();
    println!("Free memory: = {free_memory}");

    let buffer_size = (free_memory / 1000) as usize; // Use 10% of available memory for buffering * 100 bytes per message
    let (tx, rx): (Sender<(Vec<u8>, u64)>, Receiver<(Vec<u8>, u64)>) = bounded(buffer_size);

    let mut handles = Vec::new();
    let used_addresses = Arc::clone(&used_addresses);

    let file_path = args.computed_addresses;
    let mut file;
    let mut buf_reader;
    let stdin = io::stdin();

    if file_path == "" {
        // using STDIN
        buf_reader = Box::new(stdin.lock()) as Box<dyn BufRead>;
    } else {
        // using file
        file = File::open(file_path)?;
        file.seek(SeekFrom::Start(args.offset))?;
        buf_reader = Box::new(BufReader::new(file));
    }
 
    let size: usize;
    let mut offset = args.offset;

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

    println!("Using {num_threads} threads to find addresses");

    // Spawn worker threads
    for _ in 0..num_threads {
        let rx = rx.clone();
        let used_addresses = Arc::clone(&used_addresses);
        
        let handle = thread::spawn(move || {
            while let Ok((buffer, offset)) = rx.recv() {
                let address = parse_address(&buffer, args.address_format);
                if used_addresses.contains(&address) {
                    let seed = (offset - size as u64) / size as u64;
                    println!("{} {}", address, seed);
                }
            }
        });
        handles.push(handle);
    }
    
    let mut buffer = vec![0;size];

    while buf_reader.read_exact(&mut buffer).is_ok() {
        let data = buffer.clone();
        if tx.send((data, offset)).is_err() {
            break;
        }
        offset += size as u64;
        /*        
        offset += size as u64;
        let address = parse_address(&buffer, args.address_format);
  
        //println!("checking {address}");
        match used_addresses.get(&address) {
            Some(_balance) => {
                let seed = (offset - size as u64)/ size as u64;
                println!("{address}: {seed}");
            },
            None => ()
        }*/
    }
    drop(tx); // Close sender to signal workers to exit
    
    for handle in handles {
        handle.join().unwrap();
    }    
    Ok(())
}

fn parse_address(buffer: &[u8], format: common::AddressFormat) -> String {
    let mut address;
    match format {
        common::AddressFormat::ETH => {
            address = hex::encode(&buffer);
        }
        common::AddressFormat::TRX | common::AddressFormat::DOGE | common::AddressFormat::ZEC | common::AddressFormat::LTC | common::AddressFormat::BTC44 | common::AddressFormat::BTC49 => {
            address = String::from_utf8(buffer[0..buffer.len()-1].to_vec()/* .clone()*/).unwrap();
        }    
        common::AddressFormat::XRP => {
            address = String::from_utf8(buffer[0..buffer.len()-1].to_vec()/* .clone()*/).unwrap();
            address.replace_range(0..1,"r");
        }
    }

    return address;
}

fn load_set(file_path: &String, format: common::AddressFormat) -> std::io::Result<HashSet<String>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut set = HashSet::new();

    if format == common::AddressFormat::XRP {
        if let Err(err) = load_json(file_path, &mut set) {
            println!("{}", err);
            process::exit(1);
        }    
    } else if format == common::AddressFormat::BTC44 || format == common::AddressFormat::BTC49 || format == common::AddressFormat::DOGE || format == common::AddressFormat::LTC {
        if let Err(err) = load_addresses(file_path, &mut set, true, b'\t') {
            println!("{}", err);
            process::exit(1);
        }
    } else if format == common::AddressFormat::TRX {
        if let Err(err) = load_addresses(file_path, &mut set, true, b',') {
            println!("{}", err);
            process::exit(1);
        }
    } else if format == common::AddressFormat::ETH {
        if let Err(err) = load_addresses(file_path, &mut set, false, b'\t') {
            println!("{}", err);
            process::exit(1);
        }
    } else {
        if let Err(err) = load_accounts(file_path, &mut set) {
                println!("{}", err);
                process::exit(1);
        }        
    }
    println!("Used addresses loaded: {}",set.len());

    Ok(set)
}