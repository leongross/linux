#include <linux/module.h>
#include <linux/printk.h>
#include <linux/slab.h>
#include <linux/types.h>
#include <crypto/hash.h>

#include "../../lib/rust.h"

struct sdesc {
	struct shash_desc shash;
	char ctx[];
};

static struct sdesc *init_sdesc(struct crypto_shash *alg)
{
	struct sdesc *sdesc;
	int size;

	size = sizeof(struct shash_desc) + crypto_shash_descsize(alg);
	sdesc = kmalloc(size, GFP_KERNEL);
	if (!sdesc)
		return ERR_PTR(-ENOMEM);
	sdesc->shash.tfm = alg;
	return sdesc;
}

static void test_0()
{
	printk(KERN_CRIT "Calling rust dummy from C (in-tree)");
	// rust_called_from_c(int *base, size_t size);
	int arr[3] = { 0, 0, 0 };
	arr[0] = 1;
	arr[1] = 2;
	arr[2] = 3;

	printk(KERN_CRIT "0: %d, 1: %d, 2:%d", arr[0], arr[1], arr[2]);
	rust_called_from_c(&arr[0], 3);
	printk(KERN_CRIT "0: %d, 1: %d, 2:%d", arr[0], arr[1], arr[2]);
}

static void hash()
{
	printk(KERN_CRIT "Calling rust crypto function from C (in-tree)");
	char *input = (char *)kzalloc(sizeof(char) * 32, GFP_KERNEL);
	char *output = (char *)kzalloc(sizeof(char) * 32, GFP_KERNEL);
	const char *sha256 = "sha256";
	int ret = rust_hash_buffer(input, output, sha256);
	printk(KERN_CRIT "Return code of hashing: %d", ret);
	printk(KERN_CRIT "Buffer: ");
	int i;
	for (i = 0; i < 32; i++) {
		printk(KERN_CRIT "out[%d] = %02x", i, output[i]);
	}

	kfree(input);
	kfree(output);
}

static void hash_256()
{
	printk(KERN_CRIT "Calling rust crypto function from C (in-tree)");
	uint8_t *input = (uint8_t *)kzalloc(sizeof(char) * 32, GFP_KERNEL);
	uint8_t *output = (uint8_t *)kzalloc(sizeof(char) * 32, GFP_KERNEL);

	int ret = rust_hash_buffer_sha256(input, output);
	printk(KERN_CRIT "Return code of hashing: %d", ret);
	printk(KERN_CRIT "Buffer: ");
	int i;
	for (i = 0; i < 32; i++) {
		printk(KERN_CRIT "out[%d] = %02x", i, output[i]);
	}

	kfree(input);
	kfree(output);
}

static void hash_256_raw()
{
	printk(KERN_CRIT "Calling rust crypto function from C (in-tree)");
	uint8_t *input = (uint8_t *)kzalloc(sizeof(char) * 32, GFP_KERNEL);
	uint8_t *output = (uint8_t *)kzalloc(sizeof(char) * 32, GFP_KERNEL);

	// printk(KERN_CRIT "INIT BUFFERS:");
	// int j;
	// for (j = 0; j < 32; j++) {
	// 	printk(KERN_CRIT "input[%d] = %d \t output[%d] = %d", j,
	// 	       input[j], j, output[j]);
	// }

	int ret = rust_hash_buffer_sha256_raw(input, output, 32);
	printk(KERN_CRIT "Return code of hashing: %d", ret);
	printk(KERN_CRIT "Buffer: ");
	int i;
	for (i = 0; i < 32; i++) {
		//printk(KERN_CRIT "out[%d] = %u", i, output[i]);
		printk(KERN_CRIT "out[%d] = 0x%02X", i, (uint8_t)(output[i]));
	}
}
static void hash_256_raw_hack()
{
	printk(KERN_CRIT "Calling rust crypto function from C (in-tree)");
	uint8_t *input = (uint8_t *)kzalloc(sizeof(char) * 32, GFP_KERNEL);
	uint8_t *output = (uint8_t *)kzalloc(sizeof(char) * 32, GFP_KERNEL);

	// printk(KERN_CRIT "INIT BUFFERS:");
	// int j;
	// for (j = 0; j < 32; j++) {
	// 	printk(KERN_CRIT "input[%d] = %d \t output[%d] = %d", j,
	// 	       input[j], j, output[j]);
	// }

	char *hash = "sha256";
	int ret = rust_hash_buffer_raw_hack(input, output, 32, hash);
	printk(KERN_CRIT "Return code of hashing: %d", ret);
	printk(KERN_CRIT "Buffer: ");
	int i;
	for (i = 0; i < 32; i++) {
		//printk(KERN_CRIT "out[%d] = %u", i, output[i]);
		printk(KERN_CRIT "out[%d]%s= 0x%02X", i, i > 10 ? " " : "  ",
		       (uint8_t)(output[i]));
	}
	kfree(input);
	kfree(output);
}

static void hash_salt_rust()
{
	struct crypto_shash *alg;
	struct sdesc *hsdesc;
	char *hash_alg_name = "sha256";

	alg = crypto_alloc_shash(hash_alg_name, 0, 0);
	hsdesc = init_sdesc(alg);

	printk(KERN_CRIT
	       "Calling rust crypto function from C (in-tree) (%s@%s)",
	       __FILE__, __func__);
	uint8_t *input = (uint8_t *)kzalloc(sizeof(char) * 32, GFP_KERNEL);
	uint8_t *output = (uint8_t *)kzalloc(sizeof(char) * 32, GFP_KERNEL);

	unsigned char salt[] = { 0, 0 };
	crypto_shash_init(&hsdesc->shash);
	int ret = rust_calc_hash_salt_c(input, 32, output, salt, 2,
					&hsdesc->shash);

	//printk(KERN_CRIT "Return code of hashing: %d, ", ret);
	//printk(KERN_CONT "Buffer: ");
	//int i;
	//for (i = 0; i < 32; i++) {
	//	//printk(KERN_CRIT "out[%d] = %u", i, output[i]);
	//	printk(KERN_CRIT "out[%d]%s= 0x%02X", i, i > 9 ? " " : "  ",
	//	       (uint8_t)(output[i]));
	//}
	kfree(input);
	kfree(output);
}

static int __init rustfromc_init(void)
{
	//hash_256_raw();

	// stress test
	//int i;
	//for (i = 0; i < 0xffff; i++) {
	//	printk(KERN_CRIT "Execution nr. %d", i);
	//	hash_256_raw_hack();
	//}
	int i;
	for (i = 0; i < 0xFFFFFF; i++) {
		hash_salt_rust();
		printk(KERN_CRIT "Ran %d times", i);
	}

	return 0;
}

static void __exit rustfromc_exit(void)
{
	printk(KERN_CRIT "Killing rustfromc module");
}

module_init(rustfromc_init);
module_exit(rustfromc_exit);

MODULE_AUTHOR("Leon Gross <leon.gross@rub.de> <lg@edgeless.systems>");
MODULE_DESCRIPTION("Call Rust from C!");
MODULE_LICENSE("GPL");
