use clap::ValueEnum;


pub const MT_SEED_CONFIG:   u64 = 0x00010000;
pub const PERMUT_CONFIG:    u64 = 0x00020000;

pub const BTC44_CONFIG: u64 = 0x00000001;
pub const BTC49_CONFIG: u64 = 0x00000002;
pub const ETH_CONFIG:   u64 = 0x00000100;
pub const LTC_CONFIG:   u64 = 0x00000200;
pub const XRP_CONFIG:   u64 = 0x00000400;
pub const ZEC_CONFIG:   u64 = 0x00000800;
pub const DOGE_CONFIG:  u64 = 0x00001000;
pub const TRX_CONFIG:   u64 = 0x00002000;

pub const BTC44_ADDRESS_SIZE: usize = 35;
pub const BTC49_ADDRESS_SIZE: usize = 35;
pub const ETH_ADDRESS_SIZE: usize = 20;
pub const LTC_ADDRESS_SIZE: usize = 35;
pub const XRP_ADDRESS_SIZE: usize = 35;
pub const ZEC_ADDRESS_SIZE: usize = 36;
pub const DOGE_ADDRESS_SIZE: usize = 35;
pub const TRX_ADDRESS_SIZE: usize = 35;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum AddressFormat {
    /// Ethereum
    ETH,
    /// TRON
    TRX,
    /// Doge
    DOGE,
    /// ZCash
    ZEC,
    /// Litecoin
    LTC,
    /// Ripple
    XRP,
    /// BITCOIN BIP44
    BTC44,
    /// BITCOIN BIP49
    BTC49
}

pub const BTC44_FILE_NAME: &str ="BTC44.bin";
pub const BTC49_FILE_NAME: &str ="BTC49.bin";
pub const ETH_FILE_NAME: &str ="ETH.bin";
pub const LTC_FILE_NAME: &str ="LTC.bin";
pub const XRP_FILE_NAME: &str ="XRP.bin";
pub const ZEC_FILE_NAME: &str = "ZEC.bin";
pub const TRX_FILE_NAME: &str = "TRX.bin";
pub const DOGE_FILE_NAME: &str = "DOGE.bin";