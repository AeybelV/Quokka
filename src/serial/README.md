# Serial Subsystem

This folder defines and implements the kernel's SerialDevice subsystem, which provides a unified interface for serial communication across different serial based devices.

## Overview

The SerialDevice subsystem abstracts serial hardware into a common trait-based interface, allowing the kernel to interact with serial devices in a uniform way. This enables flexibility in supporting
various controllers for serial based devices without tying the kernel to a specific implementation.

## Structure

- mod.rs
  Declares and exports the core SerialDevice trait and any shared utilities or types used across drivers.

- Device Implementations
  Each supported serial controller is implemented in its own module:

## SerialDevice Trait

All serial drivers implement the SerialDevice trait, which defines the core interface:

- `init`: Device initialization
- `read_byte`: Reads a byte from the controller
- `write_byte`: Transmits a byte across the device
- `enable`: Enables the device
- `disable`: Disabled the device

## Design Notes

- The subsystem is platform-agnostic, allowing support for multiple types of serial devices (UART, SPI, etc)
- Future enhancements may include buffered I/O, interrupts, or integration with higher-level abstractions like Console, StreamDevice, UART, SPI, etc
