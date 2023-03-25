#!/bin/bash

set -e

cargo test --release
QEMU_LD_PREFIX=/usr/aarch64-linux-gnu cargo test --target aarch64-unknown-linux-gnu --release
