use std::fs;
use std::fs::{OpenOptions, File};
use std::ffi::{CString};
use ocl::{core, flags};
use ocl::enums::ArgVal;
use ocl::builders::ContextProperties;
use std::str::{self, FromStr};
use rayon::prelude::*;
use std::time::Instant;
use ocl::prm::cl_uint;
use std::io::prelude::*;
use std::ffi::{OsString};
use std::path::PathBuf;

use clap::{Parser};


/// Simple program to parse computed address against used addresses
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directory where to store the generated files
    #[arg(short, long)]
    directory_path: OsString,

    /// config
    #[arg(short, long, default_value_t = 0xFFFFFFFFFFFFFFFF)]
    config: u64,
}

fn open_or_create_file(directory_path: &OsString, filename: &str) -> File {
  let path: PathBuf = [directory_path, &OsString::from_str(filename).unwrap()].iter().collect();

  match fs::metadata(path.clone()) {
    Ok(_) => return OpenOptions::new()
                .write(true)
                .append(true)
                .open(path)
                .unwrap(),
    Err(_) => return OpenOptions::new()
                .create_new(true)
                .write(true)
                .append(true)
                .open(path)
                .unwrap(),
  }
}

fn mnemonic_gpu(platform_id: core::types::abs::PlatformId, device_id: core::types::abs::DeviceId, src: std::ffi::CString, kernel_name: &String, directory_path: &OsString, config: u64) -> ocl::core::Result<()> {

  let mut eth_file = open_or_create_file(directory_path, common::ETH_FILE_NAME);
  let mut ltc_file = open_or_create_file(directory_path, common::LTC_FILE_NAME);
  let mut xrp_file = open_or_create_file(directory_path, common::XRP_FILE_NAME);
  let mut zec_file = open_or_create_file(directory_path, common::ZEC_FILE_NAME);
  let mut trx_file = open_or_create_file(directory_path, common::TRX_FILE_NAME);
  let mut doge_file = open_or_create_file(directory_path, common::DOGE_FILE_NAME);
  
//  if config & common::BTC44_CONFIG != 0 {
    let mut btc44_file = open_or_create_file(directory_path, common::BTC44_FILE_NAME);
//  }
  
//  if config & common::BTC49_CONFIG != 0   {
    let mut btc49_file = open_or_create_file(directory_path, common::BTC49_FILE_NAME);
//  }



  let file_size = ltc_file.metadata().unwrap().len();
  let mut index:u32 = (file_size/(common::LTC_ADDRESS_SIZE as u64)) as u32;
  println!("Starting from index {index}");

  let context_properties = ContextProperties::new().platform(platform_id);
  let context = core::create_context(Some(&context_properties), &[device_id], None, None).unwrap();
  let program = core::create_program_with_source(&context, &[src]).unwrap();
  let result = core::build_program(&program, Some(&[device_id]), &CString::new(""/*"-cl-opt-disable"*/).unwrap(), None, None);
  match result {
    Err(e) => print!("Error: {e}"),
    Ok(_result) => print!("program built ok")
  }
  let queue = core::create_command_queue(&context, &device_id, None).unwrap();

  loop {
    let start = Instant::now();
    const ITEMS: u32 = 2000000;

    let mut eth_address = vec![0u8; common::ETH_ADDRESS_SIZE* (ITEMS as usize)];
    let mut ltc_address = vec![0u8; common::LTC_ADDRESS_SIZE* (ITEMS as usize)];
    let mut xrp_address = vec![0u8; common::XRP_ADDRESS_SIZE* (ITEMS as usize)];
    let mut zec_address = vec![0u8; common::ZEC_ADDRESS_SIZE* (ITEMS as usize)];
    let mut doge_address = vec![0u8; common::DOGE_ADDRESS_SIZE* (ITEMS as usize)];
    let mut trx_address = vec![0u8; common::TRX_ADDRESS_SIZE* (ITEMS as usize)];
    let mut btc44_address = vec![0u8; common::BTC44_ADDRESS_SIZE* (ITEMS as usize)];
    let mut btc49_address = vec![0u8; common::BTC49_ADDRESS_SIZE* (ITEMS as usize)];
    let start_index: cl_uint = index;

    let eth_address_buf = unsafe { core::create_buffer(&context, flags::MEM_WRITE_ONLY |
      flags::MEM_COPY_HOST_PTR, eth_address.len(), Some(&eth_address))? };

    let ltc_address_buf = unsafe { core::create_buffer(&context, flags::MEM_WRITE_ONLY |
        flags::MEM_COPY_HOST_PTR, ltc_address.len(), Some(&ltc_address))? };
    
    let xrp_address_buf = unsafe { core::create_buffer(&context, flags::MEM_WRITE_ONLY |
        flags::MEM_COPY_HOST_PTR, xrp_address.len(), Some(&xrp_address))? };
      
    let zec_address_buf = unsafe { core::create_buffer(&context, flags::MEM_WRITE_ONLY |
      flags::MEM_COPY_HOST_PTR, zec_address.len(), Some(&zec_address))? };
  
    let doge_address_buf = unsafe { core::create_buffer(&context, flags::MEM_WRITE_ONLY |
      flags::MEM_COPY_HOST_PTR, doge_address.len(), Some(&doge_address))? };

    let trx_address_buf = unsafe { core::create_buffer(&context, flags::MEM_WRITE_ONLY |
      flags::MEM_COPY_HOST_PTR, trx_address.len(), Some(&trx_address))? };

    let btc44_address_buf = unsafe { core::create_buffer(&context, flags::MEM_WRITE_ONLY |
      flags::MEM_COPY_HOST_PTR, trx_address.len(), Some(&btc44_address))? };

    let btc49_address_buf = unsafe { core::create_buffer(&context, flags::MEM_WRITE_ONLY |
      flags::MEM_COPY_HOST_PTR, trx_address.len(), Some(&btc49_address))? };
    
    let kernel = core::create_kernel(&program, kernel_name)?;

    core::set_kernel_arg(&kernel, 0, ArgVal::scalar(&start_index))?;
    core::set_kernel_arg(&kernel, 1, ArgVal::scalar(&config))?;
    core::set_kernel_arg(&kernel, 2, ArgVal::mem(&eth_address_buf))?;
    core::set_kernel_arg(&kernel, 3, ArgVal::mem(&ltc_address_buf))?;
    core::set_kernel_arg(&kernel, 4, ArgVal::mem(&xrp_address_buf))?;
    core::set_kernel_arg(&kernel, 5, ArgVal::mem(&zec_address_buf))?;
    core::set_kernel_arg(&kernel, 6, ArgVal::mem(&doge_address_buf))?;
    core::set_kernel_arg(&kernel, 7, ArgVal::mem(&trx_address_buf))?;
    core::set_kernel_arg(&kernel, 8, ArgVal::mem(&btc44_address_buf))?;
    core::set_kernel_arg(&kernel, 9, ArgVal::mem(&btc49_address_buf))?;

    unsafe { core::enqueue_kernel(&queue, &kernel, 1, None, &[ITEMS as usize,1,1],
        None, None::<core::Event>, None::<&mut core::Event>)?; }

    unsafe { core::enqueue_read_buffer(&queue, &eth_address_buf, true, 0, &mut eth_address,
                                        None::<core::Event>, None::<&mut core::Event>)?; }
    unsafe { core::enqueue_read_buffer(&queue, &ltc_address_buf, true, 0, &mut ltc_address,
                                       None::<core::Event>, None::<&mut core::Event>)?; }
    unsafe { core::enqueue_read_buffer(&queue, &xrp_address_buf, true, 0, &mut xrp_address,
                                        None::<core::Event>, None::<&mut core::Event>)?; }
    unsafe { core::enqueue_read_buffer(&queue, &zec_address_buf, true, 0, &mut zec_address,
                                        None::<core::Event>, None::<&mut core::Event>)?; }
    unsafe { core::enqueue_read_buffer(&queue, &doge_address_buf, true, 0, &mut doge_address,
                                        None::<core::Event>, None::<&mut core::Event>)?; }
    unsafe { core::enqueue_read_buffer(&queue, &trx_address_buf, true, 0, &mut trx_address,
                                        None::<core::Event>, None::<&mut core::Event>)?; }
    unsafe { core::enqueue_read_buffer(&queue, &btc44_address_buf, true, 0, &mut btc44_address,
                                        None::<core::Event>, None::<&mut core::Event>)?; }
    unsafe { core::enqueue_read_buffer(&queue, &btc49_address_buf, true, 0, &mut btc49_address,
                                        None::<core::Event>, None::<&mut core::Event>)?; }
    
    let duration = start.elapsed();

    let _ = eth_file.write_all(&eth_address);
    let _ = ltc_file.write_all(&ltc_address);
    let _ = xrp_file.write_all(&xrp_address);
    let _ = zec_file.write_all(&zec_address);
    let _ = doge_file.write_all(&doge_address);
    let _ = trx_file.write_all(&trx_address);
    let _ = btc44_file.write_all(&btc44_address);
    let _ = btc49_file.write_all(&btc49_address);

    index = index + ITEMS;

    if start_index > index 
    {
      break;
    }

    println!("{index}: Time elapsed in expensive_function() is: {:?}", duration);
  }
  return Ok(());
}


fn main() {
  let args = Args::parse();

  let platform_id = core::default_platform().unwrap();
  let device_ids = core::get_device_ids(&platform_id, Some(ocl::flags::DEVICE_TYPE_ALL/*DEVICE_TYPE_GPU*/), None).unwrap();

  let int_to_address_kernel: String = "int_to_addresses".to_string();
  let int_to_address_files = ["address", "base58", "common", "int_to_addresses", "keccak256", "mnemonic_constants", "mt19937-2", "ripemd", "secp256k1_common", "secp256k1_field", "secp256k1_group", "secp256k1_prec", "secp256k1_scalar", "secp256k1", "sha2"];

  let files = int_to_address_files;
  let kernel_name = int_to_address_kernel;

  let mut raw_cl_file = "".to_string();

  for file in &files {
    let file_path = format!("./cl/{}.cl", file);
    let file_str = fs::read_to_string(file_path).unwrap();
    raw_cl_file.push_str(&file_str);
    raw_cl_file.push_str("\n");
  }

  let src_cstring = CString::new(raw_cl_file).unwrap();

  let directory_path = args.directory_path;
  let config :u64= args.config; 
  
  device_ids.into_par_iter().for_each(move |device_id| mnemonic_gpu(platform_id, device_id, src_cstring.clone(), &kernel_name, &directory_path, config).unwrap());
}

