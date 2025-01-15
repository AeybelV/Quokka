#!/bin/bash

# Ensure the script exits on any command failure
set -e

# Default values
KERNEL_BINARY="quokka-kernel.elf"
ARCH="arm"
MACHINE="virt"
PAUSE_ON_ENTRY=""
# Help function
function usage() {
    echo "Usage: $0 [-k kernel image] [-a arch] [-m machine]"
    echo "Options:"
    echo "  -k    Kernel image (default: quokka-kernel.bin)"
    echo "  -a    arch (default: arm)"
    echo "  -m    QEMU Machine (default: virt)"
    echo "  -d    Pause the CPU at startup until GDB connects"
    exit 1
}

# Parse arguments
while getopts "k:a:m:df:" opt; do
    case ${opt} in
        k) KERNEL_BINARY=${OPTARG} ;;
        a) ARCH=${OPTARG} ;;
        m) MACHINE=${OPTARG} ;;
        d) PAUSE_ON_ENTRY="-S" ;;
        *) usage ;;
    esac
done

# Check required arguments
if [[ -z $KERNEL_BINARY || -z $ARCH ]]; then
  usage
fi

# Determine QEMU executable based on architecture
case $ARCH in
  arm) QEMU="qemu-system-arm" ;;
  aarch64) QEMU="qemu-system-aarch64" ;;
  riscv32) QEMU="qemu-system-riscv32" ;;
  riscv64) QEMU="qemu-system-riscv64" ;;
  x86_64) QEMU="qemu-system-x86_64" ;;
  *)
    echo "Unsupported architecture: $ARCH"
    exit 1
    ;;
esac

# Check if QEMU is in the PATH
if ! command -v $QEMU &> /dev/null; then
  echo "Error: $QEMU is not installed or not in the PATH."
  exit 1
fi

echo "========== STARTING UP QEMU =========="
echo "Machine: ${MACHINE}, kernel: ${KERNEL_BINARY}"
echo "Use [Ctrl-A + x] to exit QEMU"
echo "========== QEMU output below =========="

${QEMU}  \
  -machine ${MACHINE} \
  -kernel ${KERNEL_BINARY} \
  -s \
  -d int,cpu_reset \
  -serial vc \
  -monitor stdio \
  ${PAUSE_ON_ENTRY} \
