/* SPDX-License-Identifier: GPL-2.0 */
#ifndef _LIB_RUST_H
#define _LIB_RUST_H

#include <crypto/hash.h>

#ifdef CONFIG_RUST
char *rust_fmt_argument(char *buf, char *end, void *ptr);
int rust_called_from_c(int *base, size_t size);
int rust_hash_buffer(char *input, char *output, const char *hash);
int rust_hash_buffer_sha256(char *input, char *output);
int rust_hash_buffer_sha256_raw(char *input, char *output, int len);
int rust_hash_buffer_raw_hack(unsigned char *input, unsigned char *output,
			      unsigned int len, char *hash);

int rust_calc_hash_salt_c(unsigned char *data, unsigned int data_len,
			  unsigned char *out, unsigned char *salt,
			  unsigned int salt_size, struct shash_desc *sdesc);
#else

static inline char *rust_fmt_argument(char *buf, char *end, void *ptr)
{
	return NULL;
}

static inline int rust_called_from_c(int *base, size_t size)
{
	return NULL;
}

static int rust_hash_buffer(void *input, void *output, char *hash)
{
	return NULL;
}
#endif

#endif /* _LIB_RUST_H */
