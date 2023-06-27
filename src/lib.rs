#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

//! Enable or disable low latency mode for serial ports of type serialport::TTYPort on Linux.

mod ioctls {
    use nix::{ioctl_read_bad, ioctl_write_ptr_bad};

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

    ioctl_read_bad!(tiocgserial, TIOCGSERIAL, serial_struct);
    ioctl_write_ptr_bad!(tiocsserial, TIOCSSERIAL, serial_struct);
}

use ioctls::{serial_struct, tiocgserial, tiocsserial, ASYNC_LOW_LATENCY};
use std::mem::MaybeUninit;
use std::os::unix::prelude::AsRawFd;

enum UpdateAction {
    Enable,
    Disable,
}

fn update_low_latency(port: &mut serialport::TTYPort, action: UpdateAction) -> nix::Result<()> {
    let raw_fd = port.as_raw_fd();
    let mut serial_line_info = MaybeUninit::<serial_struct>::uninit();

    // Safety:
    // * serial_line_info.as_mut_ptr: The pointer is used by tiocgserial to initialize the memory.
    //   The pointer is not read or turned into a reference here.
    // * tiocgserial: There are no safety remarks in the documentation of nix::sys::ioctl. However,
    //   * raw_fd is a valid file descriptor, pointing to a TTY device (port still exists), and
    //   * the pointer passed to the TIOCGSERIAL ioctl points to an instance of type serial_struct.
    unsafe { tiocgserial(raw_fd, serial_line_info.as_mut_ptr()) }?;

    // Safety: serial_line_info has been initialized by the TIOCGSERIAL ioctl.
    let mut serial_line_info = unsafe { serial_line_info.assume_init() };

    match action {
        UpdateAction::Enable => {
            serial_line_info.flags = serial_line_info.flags | (ASYNC_LOW_LATENCY as i32)
        }
        UpdateAction::Disable => {
            serial_line_info.flags = serial_line_info.flags & !(ASYNC_LOW_LATENCY as i32)
        }
    }

    // Safety: There are no safety remarks in the documentation of nix::sys::ioctl. However,
    // * raw_fd is a valid file descriptor, pointing to a TTY device (port still exists), and
    // * the pointer passed to the TIOCSSERIAL ioctl points to an instance of type serial_struct.
    unsafe { tiocsserial(raw_fd, &serial_line_info) }?;

    Ok(())
}

/// Enable low latency mode for the given serial port.
///
/// # Example
/// Open a serial port and enable low latency mode
/// ```
/// use std::time::Duration;
/// use serialport_low_latency::enable_low_latency;
/// let mut port = serialport::new("/dev/ttyUSB0", 115_200)
///     .timeout(Duration::from_millis(10))
///     .open_native().expect("Failed to open port");
/// enable_low_latency(&mut port).unwrap();
/// ```
pub fn enable_low_latency(port: &mut serialport::TTYPort) -> nix::Result<()> {
    update_low_latency(port, UpdateAction::Enable)
}

/// Disable low latency mode for the given serial port.
///
/// # Example
/// ```
/// use std::time::Duration;
/// use serialport_low_latency::disable_low_latency;
/// let mut port = serialport::new("/dev/ttyUSB0", 115_200)
///     .timeout(Duration::from_millis(10))
///     .open_native().expect("Failed to open port");
/// disable_low_latency(&mut port).unwrap();
/// ```
pub fn disable_low_latency(port: &mut serialport::TTYPort) -> nix::Result<()> {
    update_low_latency(port, UpdateAction::Disable)
}

#[cfg(test)]
mod tests {
    #[ignore]
    #[test]
    fn unified_test() {
        // This test requires /dev/ttyUSB0 to exist and to be a compatible FTDI chip

        use crate::{disable_low_latency, enable_low_latency};
        use std::fs;
        use std::time::Duration;

        let path = "/sys/bus/usb-serial/devices/ttyUSB0/latency_timer";
        let mut port = serialport::new("/dev/ttyUSB0", 115_200)
            .timeout(Duration::from_millis(10))
            .open_native()
            .unwrap();

        // Enable
        enable_low_latency(&mut port).unwrap();
        let latency = fs::read_to_string(path).unwrap();
        assert_eq!(latency.trim(), "1");

        // Disable
        disable_low_latency(&mut port).unwrap();
        let latency = fs::read_to_string(path).unwrap();
        assert_eq!(latency.trim(), "16");

        // Enable
        enable_low_latency(&mut port).unwrap();
        let latency = fs::read_to_string(path).unwrap();
        assert_eq!(latency.trim(), "1");

        // Disable
        disable_low_latency(&mut port).unwrap();
        let latency = fs::read_to_string(path).unwrap();
        assert_eq!(latency.trim(), "16");
    }
}
