#include <linux/module.h>
#include "../../lib/rust.h"

static int __init rustfromc_init(void)
{
    printk(KERN_CRIT "Calling rust function from C!");
    // rust_called_from_c(int *base, size_t size);
    int arr[3] = {0,0,0};
    arr[0] = 1;
    arr[1] = 2;
    arr[2] = 3;

    printk(KERN_CRIT "0: %d, 1: %d, 2:%d", arr[0], arr[1], arr[2]);
    rust_called_from_c(&arr[0], 3);
    printk(KERN_CRIT "0: %d, 1: %d, 2:%d", arr[0], arr[1], arr[2]);
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
