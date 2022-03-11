#include <linux/kernel.h>
#include <linux/module.h>
#include <linux/init.h>
#include "../../lib/rust.h"

MODULE_LICENSE("GPL");

static int rust2c_init(void) {
    int arr[4];
    int sum;

    arr[0] = 1;
    arr[1] = 3;
    arr[2] = 3;
    arr[3] = 7;

    sum = rust_called_from_c(arr, 4);
    printk(KERN_INFO "sum: %d\n", sum);
    printk(KERN_INFO "Items: [%d, %d, %d, %d]\n", arr[0], arr[1], arr[2], arr[3]);

    return 0;
}

static void rust2c_exit(void) {
    printk(KERN_INFO "Exiting kernel module :)");
}

module_init(rust2c_init);
module_exit(rust2c_exit);
