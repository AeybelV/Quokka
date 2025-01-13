#!/bin/bash

# Ensure the script exits on any command failure
set -e

# Default values
RUST_TARGET="armv7a-none-eabi"
BUILD_MODE="release"
OUTPUT_DIR="target/${RUST_TARGET}/${BUILD_MODE}"
OUTPUT_FILE="quokka-kernel"
ABI="arm-none-eabi" # Default abi
OBJCOPY="${ABI}-objcopy"

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
while getopts "t:a:m:" opt; do
    case ${opt} in
        t) RUST_TARGET=${OPTARG} ;;
        a) ABI=${OPTARG} ;;
        m) BUILD_MODE=${OPTARG} ;;
        *) usage ;;
    esac
done

# Validate build mode
if [[ "${BUILD_MODE}" != "release" && "${BUILD_MODE}" != "debug" ]]; then
    echo "Invalid build mode: ${BUILD_MODE}"
    usage
fi

# Build the Rust project
echo "Building for Rust target: ${RUST_TARGET}, mode: ${BUILD_MODE}"
if [[ "${BUILD_MODE}" == "release" ]]; then
    cargo build --release --target "${RUST_TARGET}" --verbose
else
    cargo build --target "${RUST_TARGET}" --verbose
fi

# Ensure the output directory exists
if [[ ! -d "${OUTPUT_DIR}" ]]; then
    echo "Error: Build output directory does not exist: ${OUTPUT_DIR}"
    exit 1
fi

# Locate the ELF file
ELF_FILE="${OUTPUT_DIR}/quokka"
if [[ ! -f "${ELF_FILE}" ]]; then
    echo "Error: ELF file not found: ${ELF_FILE}"
    exit 1
fi

# Convert the ELF file to binary
echo "Converting ELF to binary using ${OBJCOPY}"
timestamp=$(date --utc +%Y%m%d_%H%M%SZ)

KERNEL_BUILD_DIR="kernel_output/${RUST_TARGET}"
KERNEL_OUTPUT="${KERNEL_BUILD_DIR}/${OUTPUT_FILE}-${RUST_TARGET}_${timestamp}.bin"
KERNEL_OUTPUT_ELF="${KERNEL_BUILD_DIR}/${OUTPUT_FILE}-${RUST_TARGET}_${timestamp}.elf"
mkdir -p ${KERNEL_BUILD_DIR}

${OBJCOPY} -O binary "${ELF_FILE}" "${KERNEL_OUTPUT}"
cp "${ELF_FILE}" "${KERNEL_OUTPUT_ELF}"

echo "Kernel ELF created: ${KERNEL_OUTPUT_ELF}"
echo "Kernel binary created: ${KERNEL_OUTPUT}"
ln -sf ${KERNEL_OUTPUT} ${OUTPUT_FILE}.bin
ln -sf ${KERNEL_OUTPUT_ELF} ${OUTPUT_FILE}.elf
echo "Latest symlink: ${OUTPUT_FILE}.bin"
echo "Latest symlink: ${OUTPUT_FILE}.elf"
