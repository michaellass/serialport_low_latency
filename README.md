# serialport_low_latency

[![license](https://img.shields.io/github/license/pc2/serialport_low_latency.svg)](https://github.com/pc2/serialport_low_latency/blob/master/LICENSE)
[![crates.io](https://img.shields.io/crates/v/serialport_low_latency.svg)](https://crates.io/crates/serialport_low_latency)
[![docs.rs](https://docs.rs/serialport_low_latency/badge.svg)](https://docs.rs/serialport_low_latency)

FTDI serial communication chips support a low latency mode where the latency timer is reduced to
1 ms. This package allows enabling and disabling this low latency mode on Linux via the TIOCSSERIAL
ioctl.

## Examples

### Open a serial port and enable low latency mode
```rust
use std::time::Duration;
use serialport_low_latency::enable_low_latency;
let mut port = serialport::new("/dev/ttyUSB0", 115_200)
    .timeout(Duration::from_millis(10))
    .open_native().expect("Failed to open port");
enable_low_latency(&mut port).unwrap();
```

### Open a serial port and disable low latency mode
```rust
use std::time::Duration;
use serialport_low_latency::disable_low_latency;
let mut port = serialport::new("/dev/ttyUSB0", 115_200)
    .timeout(Duration::from_millis(10))
    .open_native().expect("Failed to open port");
disable_low_latency(&mut port).unwrap();
```
