Minimal development setup
=========================

This document describes how to set up a minimal development environment to get started.

IDE: rust-analyzer
------------------
``rust-analyzer`` is modular compiler fronted for the Rust language.
To make the development more comfortable the the project provides a Makefile target, which produces a configuration fle for ``rust-analyzer``.
The file ````` be in the root directory of the kernel source code.


Running: QEMU
-------------
To boot the compiled kernel a few things are required (See ``ci.yaml`` for reference).
For this example we assume ``ARCH=x86_64``

Crrea


Optional but recommended:
1. Initramfs
