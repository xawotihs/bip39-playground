constant uchar B58_DIGITS_ORDERED[] = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
constant uchar B58_DIGITS_RIPPLE[] = "rpshnaf39wBUDNEGHJKLM4PQRST7VWXYZ2bcdeCg65jkm8oFqi1tuvAxyz";

bool b58enc (__global uchar *b58, ulong *b58sz, const uchar *data, const ulong binsz)
{
  const uchar *bin = data;

  int carry;
  ulong j      = 0;
  ulong zcount = 0;

  while (zcount < binsz && !bin[zcount]) ++zcount;

  ulong size = (binsz - zcount) * 138 / 100 + 1;

  uchar buf[200] = { 0 };

  ulong i    = zcount;
  ulong high = size - 1;

  for (; i < binsz; i++, high = j)
  {
    for (carry = bin[i], j = size - 1; (j > high) || carry; j--)
    {
      carry += 256 * buf[j];

      buf[j] = carry % 58;

      carry /= 58;

      if (! j) break;
    }
  }

  j = 0;

  for (; j < size && !buf[j]; j++) {}

  if (*b58sz <= zcount + size - j)
  {
    *b58sz = zcount + size - j + 1;

    return false;
  }

  for (ulong i = 0; i < zcount; i++)
  {
    b58[i] = '1';
  }

  for (i = zcount; j < size; i++, j++)
  {
    b58[i] = B58_DIGITS_ORDERED[buf[j]];
  }

  b58[i] = '\0';

  *b58sz = i + 1;

  return true;
}

bool b58enc_ripple (__global uchar *b58, ulong *b58sz, const uchar *data, const ulong binsz)
{
  const uchar *bin = data;

  int carry;
  ulong j      = 0;
  ulong zcount = 0;

  while (zcount < binsz && !bin[zcount]) ++zcount;

  ulong size = (binsz - zcount) * 138 / 100 + 1;

  uchar buf[200] = { 0 };

  ulong i    = zcount;
  ulong high = size - 1;

  for (; i < binsz; i++, high = j)
  {
    for (carry = bin[i], j = size - 1; (j > high) || carry; j--)
    {
      carry += 256 * buf[j];

      buf[j] = carry % 58;

      carry /= 58;

      if (! j) break;
    }
  }

  j = 0;

  for (; j < size && !buf[j]; j++) {}

  if (*b58sz <= zcount + size - j)
  {
    *b58sz = zcount + size - j + 1;

    return false;
  }

  for (ulong i = 0; i < zcount; i++)
  {
    b58[i] = 'r';
  }

  for (i = zcount; j < size; i++, j++)
  {
    b58[i] = B58_DIGITS_RIPPLE[buf[j]];
  }

  b58[i] = '\0';

  *b58sz = i + 1;

  return true;
}