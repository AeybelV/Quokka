#!/bin/bash

# Copyright (c) 2025 Aeybel Varghese
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.
# Ensure the script exits on any command failure

set -e

# Default values
KERNEL_BINARY="quokka-kernel.*"
OUTPUT_IMG_SIZE=40

# Help function
function usage() {
    echo "Usage: $0 [-t rust_target] [-a abi] [-m mode]"
    echo "Options:"
    echo "  -t    Rust target (default: armv7a-none-eabi)"
    echo "  -a    abi (default: arm-none-eabi)"
    echo "  -m    Build mode (release or debug, default: release)"
    exit 1
}

# Parse arguments
while getopts "k:s:" opt; do
    case ${opt} in
        k) KERNEL_BINARY=${OPTARG} ;;
        s) OUTPUT_IMG_SIZE=${OPTARG} ;;
        *) usage ;;
    esac
done

timestamp=$(date --utc +%Y%m%d_%H%M%SZ)
SD_OUTPUT_DIR="sd_output"

OUTPUT_IMG="${SD_OUTPUT_DIR}/quokka_${timestamp}.img"
TEMP_MOUNT_DIR="$(mktemp -d)"

mkdir -p ${SD_OUTPUT_DIR}
dd if=/dev/null of=${OUTPUT_IMG} bs=1M seek=${OUTPUT_IMG_SIZE}
mkfs.fat -F 32 ${OUTPUT_IMG}
sudo mount -t vfat -o loop ${OUTPUT_IMG} ${TEMP_MOUNT_DIR}

# construct rootfs
sudo cp ${KERNEL_BINARY} ${TEMP_MOUNT_DIR}

sudo umount ${TEMP_MOUNT_DIR}

echo "SD Image created: ${OUTPUT_IMG}"
ln -sf ${OUTPUT_IMG} "quokka-sd.img"
echo "Latest symlink: quokka-sd.img"
