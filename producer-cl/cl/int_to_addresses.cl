
void idx_to_mt_seed128(uint idx, ulong *mnemonic_lo, ulong *mnemonic_hi)
{
  ulong value;

  mt19937_state mtstate;
  mt19937_seed(&mtstate, idx);

  *mnemonic_hi = 0;
  *mnemonic_lo = 0;

  uint i = 0;
  while(i<8)
  {
    value = mt19937_uint(&mtstate);
    value &= 0x00000000000000ff;
    value = ((value) << ((7-i)*8));
//    printf("%u\n", value);
    *mnemonic_hi += value;
    i++;
  }

  i = 0;
  while(i<8)
  {
    value = mt19937_uint(&mtstate);
    value &= 0x00000000000000ff;
    value = ((value) << ((7-i)*8));
//    printf("%u\n", value);
   *mnemonic_lo += value;
   i++;
  }
}

void seed128_to_mnemonic(ulong mnemonic_lo, ulong mnemonic_hi, uchar *mnemonic, uchar*mnemonic_length)
{

  uchar bytes[16];

  bytes[15] = mnemonic_lo & 0xFF;
  bytes[14] = (mnemonic_lo >> 8) & 0xFF;
  bytes[13] = (mnemonic_lo >> 16) & 0xFF;
  bytes[12] = (mnemonic_lo >> 24) & 0xFF;
  bytes[11] = (mnemonic_lo >> 32) & 0xFF;
  bytes[10] = (mnemonic_lo >> 40) & 0xFF;
  bytes[9] = (mnemonic_lo >> 48) & 0xFF;
  bytes[8] = (mnemonic_lo >> 56) & 0xFF;
  
  bytes[7] = mnemonic_hi & 0xFF;
  bytes[6] = (mnemonic_hi >> 8) & 0xFF;
  bytes[5] = (mnemonic_hi >> 16) & 0xFF;
  bytes[4] = (mnemonic_hi >> 24) & 0xFF;
  bytes[3] = (mnemonic_hi >> 32) & 0xFF;
  bytes[2] = (mnemonic_hi >> 40) & 0xFF;
  bytes[1] = (mnemonic_hi >> 48) & 0xFF;
  bytes[0] = (mnemonic_hi >> 56) & 0xFF;

  uchar mnemonic_hash[32];
  sha256(bytes, 16, &mnemonic_hash);
  //uchar checksum = mnemonic_hash[0] >> 4;
  uchar checksum = (mnemonic_hash[0] >> 4) & ((1 << 4)-1);

  ushort indices[12];
  indices[0] = (mnemonic_hi & ((ulong)2047 << 53)) >> 53;
  indices[1] = (mnemonic_hi & ((ulong)2047 << 42)) >> 42;
  indices[2] = (mnemonic_hi & ((ulong)2047 << 31)) >> 31;
  indices[3] = (mnemonic_hi & ((ulong)2047 << 20)) >> 20;
  indices[4] = (mnemonic_hi & ((ulong)2047 << 9)) >> 9;
  indices[5] = ((mnemonic_hi << 55) >> 53) | ((mnemonic_lo & (3 << 62)) >> 62);
  indices[6] = (mnemonic_lo & ((ulong)2047 << 51)) >> 51;
  indices[7] = (mnemonic_lo & ((ulong)2047 << 40)) >> 40;
  indices[8] = (mnemonic_lo & ((ulong)2047 << 29)) >> 29;
  indices[9] = (mnemonic_lo & ((ulong)2047 << 18)) >> 18;
  indices[10] = (mnemonic_lo & ((ulong)2047 << 7)) >> 7;
  indices[11] = ((mnemonic_lo << 57) >> 53) | checksum;

  *mnemonic_length = 11 + word_lengths[indices[0]] + word_lengths[indices[1]] + word_lengths[indices[2]] + word_lengths[indices[3]] + word_lengths[indices[4]] + word_lengths[indices[5]] + word_lengths[indices[6]] + word_lengths[indices[7]] + word_lengths[indices[8]] + word_lengths[indices[9]] + word_lengths[indices[10]] + word_lengths[indices[11]];

  int mnemonic_index = 0;
  
  for (int i=0; i < 12; i++) {
    int word_index = indices[i];
    int word_length = word_lengths[word_index];
    
    for(int j=0;j<word_length;j++) {
      mnemonic[mnemonic_index] = words[word_index][j];
      mnemonic_index++;
    }
    mnemonic[mnemonic_index] = 32;
    mnemonic_index++;
  }
  mnemonic[mnemonic_index - 1] = 0;
}

void mnemonic_to_seed(const uchar* mnemonic, const uchar mnemonic_length, uchar* seed)
{
  uchar ipad_key[128];
  uchar opad_key[128];
  for(int x=0;x<128;x++){
    ipad_key[x] = 0x36;
    opad_key[x] = 0x5c;
  }

  for(int x=0;x<mnemonic_length;x++){
    ipad_key[x] = ipad_key[x] ^ mnemonic[x];
    opad_key[x] = opad_key[x] ^ mnemonic[x];
  }

  uchar sha512_result[64] = { 0 };
  uchar key_previous_concat[256] = { 0 };
  uchar salt[12] = { 109, 110, 101, 109, 111, 110, 105, 99, 0, 0, 0, 1 };
  for(int x=0;x<128;x++){
    key_previous_concat[x] = ipad_key[x];
  }
  for(int x=0;x<12;x++){
    key_previous_concat[x+128] = salt[x];
  }

  sha512(&key_previous_concat, 140, &sha512_result);
  copy_pad_previous(&opad_key, &sha512_result, &key_previous_concat);
  sha512(&key_previous_concat, 192, &sha512_result);
  xor_seed_with_round(seed, &sha512_result);

  for(int x=1;x<2048;x++){
    copy_pad_previous(&ipad_key, &sha512_result, &key_previous_concat);
    sha512(&key_previous_concat, 192, &sha512_result);
    copy_pad_previous(&opad_key, &sha512_result, &key_previous_concat);
    sha512(&key_previous_concat, 192, &sha512_result);
    xor_seed_with_round(seed, &sha512_result);
  }
}

void private_key_to_BTC49_address(extended_private_key_t* master_private, __global uchar* raw_address)
{
  extended_private_key_t target_key;
  extended_public_key_t target_public_key;
  hardened_private_child_from_private(master_private, &target_key, 49);
  hardened_private_child_from_private(&target_key, &target_key, 0);
  hardened_private_child_from_private(&target_key, &target_key, 0);
  normal_private_child_from_private(&target_key, &target_key, 0);
  normal_private_child_from_private(&target_key, &target_key, 0);
  public_from_private(&target_key, &target_public_key);

  uchar identifier[25] = { 0 };
  p2shwpkh_address_for_public_key(&target_public_key, &identifier);
  ulong output_size = 35;
  b58enc (raw_address, &output_size, identifier, 25);
}

void private_key_to_BTC44_address(extended_private_key_t* master_private, __global uchar* raw_address)
{
  extended_private_key_t target_key;
  extended_public_key_t target_public_key;
  hardened_private_child_from_private(master_private, &target_key, 44);
  hardened_private_child_from_private(&target_key, &target_key, 0);
  hardened_private_child_from_private(&target_key, &target_key, 0);
  normal_private_child_from_private(&target_key, &target_key, 0);
  normal_private_child_from_private(&target_key, &target_key, 0);
  public_from_private(&target_key, &target_public_key);

  uchar identifier[25] = { 0 };
  identifier[0] = 0x0;
  identifier_for_public_key(&target_public_key, (&identifier[1]));

  uchar sha256d_result[32] = { 0 };
  sha256d(identifier, 21, sha256d_result);

  identifier[21] = sha256d_result[0];
  identifier[22] = sha256d_result[1];
  identifier[23] = sha256d_result[2];
  identifier[24] = sha256d_result[3];

  ulong output_size = 35;
  b58enc (raw_address, &output_size, identifier, 25);
}

void private_key_to_ETH_address(extended_private_key_t* master_private, __global uchar* raw_address)
{
  extended_private_key_t target_key;
  extended_public_key_t target_public_key;
  hardened_private_child_from_private(master_private, &target_key, 44);
  hardened_private_child_from_private(&target_key, &target_key, 60);
  hardened_private_child_from_private(&target_key, &target_key, 0);
  normal_private_child_from_private(&target_key, &target_key, 0);
  normal_private_child_from_private(&target_key, &target_key, 0);
  public_from_private(&target_key, &target_public_key);

  uchar uncompressed_public[65];
  uchar keccak_hash[64];
  uncompressed_public_key(&target_public_key, uncompressed_public);
  //print_byte_array_hex(uncompressed_public, 65);
  
  SHA3_CTX ctx;
  keccak_init(&ctx);
  keccak_update(&ctx, &(uncompressed_public[1]), 64);
  keccak_final(&ctx, keccak_hash);
  //print_byte_array_hex(&(keccak_hash[12]), 20);

  for(int i=0;i<20;i++){
    raw_address[i] = keccak_hash[12+i];
  }  
}

void private_key_to_TRX_address(extended_private_key_t* master_private, __global uchar* raw_address)
{
  extended_private_key_t target_key;
  extended_public_key_t target_public_key;
  hardened_private_child_from_private(master_private, &target_key, 44);
  hardened_private_child_from_private(&target_key, &target_key, 195);
  hardened_private_child_from_private(&target_key, &target_key, 0);
  normal_private_child_from_private(&target_key, &target_key, 0);
  normal_private_child_from_private(&target_key, &target_key, 0);
  public_from_private(&target_key, &target_public_key);

  uchar uncompressed_public[65];
  uchar keccak_hash[64];
  uncompressed_public_key(&target_public_key, uncompressed_public);
  //print_byte_array_hex(uncompressed_public, 65);
  
  SHA3_CTX ctx;
  keccak_init(&ctx);
  keccak_update(&ctx, &(uncompressed_public[1]), 64);
  keccak_final(&ctx, keccak_hash);
  //print_byte_array_hex(&(keccak_hash[12]), 20);

  uchar identifier[25] = { 0 };
  identifier[0] = 0x41;
  for(int i=0;i<20;i++){
    identifier[i+1] = keccak_hash[12+i];
  }

  uchar sha256d_result[32] = { 0 };
  sha256d(identifier, 21, sha256d_result);

  identifier[21] = sha256d_result[0];
  identifier[22] = sha256d_result[1];
  identifier[23] = sha256d_result[2];
  identifier[24] = sha256d_result[3];

  ulong output_size = 35;
  b58enc (raw_address, &output_size, identifier, 25);
  //printf("%s\n", raw_address);
}

void private_key_to_LTC_address(extended_private_key_t* master_private, __global uchar* raw_address)
{
  extended_private_key_t target_key;
  extended_public_key_t target_public_key;
  hardened_private_child_from_private(master_private, &target_key, 44);
  hardened_private_child_from_private(&target_key, &target_key, 2);
  hardened_private_child_from_private(&target_key, &target_key, 0);
  normal_private_child_from_private(&target_key, &target_key, 0);
  normal_private_child_from_private(&target_key, &target_key, 0);
  public_from_private(&target_key, &target_public_key);

  uchar identifier[25] = { 0 };
  identifier[0] = 0x30;
  identifier_for_public_key(&target_public_key, (&identifier[1]));

  uchar sha256d_result[32] = { 0 };
  sha256d(identifier, 21, sha256d_result);

  identifier[21] = sha256d_result[0];
  identifier[22] = sha256d_result[1];
  identifier[23] = sha256d_result[2];
  identifier[24] = sha256d_result[3];

  ulong output_size = 35;
  b58enc (raw_address, &output_size, identifier, 25);

  //printf("%s\n", raw_address);
}

void private_key_to_XRP_address(extended_private_key_t* master_private, __global uchar* raw_address)
{
  extended_private_key_t target_key;
  extended_public_key_t target_public_key;
  hardened_private_child_from_private(master_private, &target_key, 44);
  hardened_private_child_from_private(&target_key, &target_key, 144);
  hardened_private_child_from_private(&target_key, &target_key, 0);
  normal_private_child_from_private(&target_key, &target_key, 0);
  normal_private_child_from_private(&target_key, &target_key, 0);
  public_from_private(&target_key, &target_public_key);

  uchar identifier[25] = { 0 };
  identifier[0] = 0x00;
  identifier_for_public_key(&target_public_key, (&identifier[1]));

  uchar sha256d_result[32] = { 0 };
  sha256d(identifier, 21, sha256d_result);

  identifier[21] = sha256d_result[0];
  identifier[22] = sha256d_result[1];
  identifier[23] = sha256d_result[2];
  identifier[24] = sha256d_result[3];

  ulong output_size = 35;
  b58enc_ripple (raw_address, &output_size, identifier, 25);

  //printf("%s\n", raw_address);
}

void private_key_to_DOGE_address(extended_private_key_t* master_private, __global uchar* raw_address)
{
  extended_private_key_t target_key;
  extended_public_key_t target_public_key;
  hardened_private_child_from_private(master_private, &target_key, 44);
  hardened_private_child_from_private(&target_key, &target_key, 3);
  hardened_private_child_from_private(&target_key, &target_key, 0);
  normal_private_child_from_private(&target_key, &target_key, 0);
  normal_private_child_from_private(&target_key, &target_key, 0);
  public_from_private(&target_key, &target_public_key);

  uchar identifier[25] = { 0 };
  identifier[0] = 0x1E;
  identifier_for_public_key(&target_public_key, (&identifier[1]));

  uchar sha256d_result[32] = { 0 };
  sha256d(identifier, 21, sha256d_result);

  identifier[21] = sha256d_result[0];
  identifier[22] = sha256d_result[1];
  identifier[23] = sha256d_result[2];
  identifier[24] = sha256d_result[3];

  ulong output_size = 35;
  b58enc (raw_address, &output_size, identifier, 25);

  //printf("%s\n", raw_address);
}

void private_key_to_ZEC_address(extended_private_key_t* master_private, __global uchar* raw_address)
{
  extended_private_key_t target_key;
  extended_public_key_t target_public_key;
  hardened_private_child_from_private(master_private, &target_key, 44);
  hardened_private_child_from_private(&target_key, &target_key, 133);
  hardened_private_child_from_private(&target_key, &target_key, 0);
  normal_private_child_from_private(&target_key, &target_key, 0);
  normal_private_child_from_private(&target_key, &target_key, 0);
  public_from_private(&target_key, &target_public_key);

  uchar identifier[26] = { 0 };
  identifier[0] = 0x1C;
  identifier[1] = 0xB8;
  identifier_for_public_key(&target_public_key, (&identifier[2]));

  uchar sha256d_result[32] = { 0 };
  sha256d(identifier, 22, sha256d_result);

  identifier[22] = sha256d_result[0];
  identifier[23] = sha256d_result[1];
  identifier[24] = sha256d_result[2];
  identifier[25] = sha256d_result[3];

  ulong output_size = 36;
  b58enc (raw_address, &output_size, identifier, 26);

  //printf("%s\n", raw_address);
}

#define BTC44_CONFIG  0x00000001
#define BTC49_CONFIG  0x00000002
#define ETH_CONFIG    0x00000100
#define LTC_CONFIG    0x00000200
#define XRP_CONFIG    0x00000400
#define ZEC_CONFIG    0x00000800
#define DOGE_CONFIG   0x00001000
#define TRX_CONFIG    0x00002000

__kernel void int_to_addresses(uint start_index, ulong config, __global uchar * eth_address, __global uchar * ltc_address, __global uchar * xrp_address, __global uchar * zec_address, __global uchar * doge_address, __global uchar * trx_address, __global uchar * btc44_address, __global uchar * btc49_address) {
  uint idx = get_global_id(0);

  ulong mnemonic_lo;
  ulong mnemonic_hi;
  uchar mnemonic[180] = {0};
  uchar mnemonic_length;

  
  if (config & MT_SEED_CONFIG) {
    // random is used from a seed equal to start_index+idx
    idx_to_mt_seed128(start_index + idx, &mnemonic_lo, &mnemonic_hi);
    seed128_to_mnemonic(mnemonic_lo, mnemonic_hi, mnemonic, &mnemonic_length);

    //printf("idx %u: %s\n", start_index + idx, mnemonic);
  } else if (config & INDICES_CONFIG) {
    // input indices are used in combination to 44 bits of (start_index&0x3FFFFF) and (idx&0x3FFFFF) to set 4 among the 12 empty indices

  }

  uchar seed[64] = { 0 };
  mnemonic_to_seed(mnemonic, mnemonic_length, seed);

  //print_byte_array_hex(seed, 64);

  uchar network = BITCOIN_MAINNET;
  extended_private_key_t master_private;
  extended_public_key_t master_public;

  new_master_from_seed(network, &seed, &master_private);
  
  if(config & ETH_CONFIG)
    private_key_to_ETH_address(&master_private, eth_address+(idx*20));
  if(config & LTC_CONFIG)
    private_key_to_LTC_address(&master_private, ltc_address+(idx*35));
  if(config & XRP_CONFIG)
    private_key_to_XRP_address(&master_private, xrp_address+(idx*35));
  if(config & ZEC_CONFIG)
    private_key_to_ZEC_address(&master_private, zec_address+(idx*36));
  if(config & DOGE_CONFIG)
    private_key_to_DOGE_address(&master_private, doge_address+(idx*35));
  if(config & TRX_CONFIG)
    private_key_to_TRX_address(&master_private, trx_address+(idx*35));
  if(config & BTC44_CONFIG)
    private_key_to_BTC44_address(&master_private, btc44_address+(idx*35));
  if(config & BTC49_CONFIG)
    private_key_to_BTC49_address(&master_private, btc49_address+(idx*35));
}
