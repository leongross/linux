#!/usr/bin/env bash

# https://01.org/linuxgraphics/gfx-docs/drm/dev-tools/gdb-kernel-debugging.html
# https://nickdesaulniers.github.io/blog/2018/10/24/booting-a-custom-linux-kernel-in-qemu-and-debugging-it-with-gdb/
# https://uaf.io/exploitation/misc/2016/09/10/Kernel-Exploitation-for-Dummies.html
KDIR=../
GDB=1
DEBUG=0

function debug {
    declare -A GDB_ARGS
    GDB_ARGS[CONFIG_GDB_SCRIPTS]="y"
    GDB_ARGS[CONFIG_DEBUG_INFO_REDUCED]="is not set"
    GDB_ARGS[SONFIG_FRAME_POINTER]="y"


    # check if all kernel config args are set correctly
    if [[ -n "$GDB"  ]];then
        for config in "${!GDB_ARGS[@]}";do
            if ! grep -irq "$config=${GDB_ARGS[$config]}" "$KDIR.config" ;then
                echo "config $config not found or false in map"
            fi	
        done
    fi

    sleep 3

    qemu-system-x86_64 \
        -kernel ../arch/x86_64/boot/bzImage \
        -initrd qemu-initramfs.img \
        -nographic \
        -m 512 \
        --append "console=ttyS0 nokaslr" \
        -s \
        -S \
        --enable-kvm
}

function interactive {
    qemu-system-x86_64 \
        -kernel ../arch/x86_64/boot/bzImage \
        -initrd qemu-initramfs.img \
        -nographic \
        -no-reboot \
        -m 512 \
        --append "console=ttyS0 nokaslr" \
        --enable-kvm
}

if [[ "$DEBUG" -eq 1 ]];then
    set -x
    debug
else
    interactive
fi
