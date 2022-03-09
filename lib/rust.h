/* SPDX-License-Identifier: GPL-2.0 */
#ifndef _LIB_RUST_H
#define _LIB_RUST_H

#ifdef CONFIG_RUST
char *rust_fmt_argument(char* buf, char* end, void *ptr);
int rust_called_from_c(int *base, size_t size);
#else
static inline char *rust_fmt_argument(char* buf, char* end, void *ptr)
{
	return NULL;
}
#endif

#endif /* _LIB_RUST_H */
