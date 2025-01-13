pub mod serial;

#[cfg(feature = "pl011")]
pub mod amba_pl011;

#[cfg(feature = "system_console_pl011")]
pub type SystemConsole = amba_pl011::PL011Uart;
