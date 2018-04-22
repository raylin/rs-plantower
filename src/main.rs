extern crate serialport;
extern crate time;

use std::io;
use std::time::Duration;
use serialport::prelude::*;

const BYTE_NUMS: usize = 24;

fn process_bytes(bytes: &[u8]) {
    let hex = bytes
        .into_iter()
        .map(|b| format!("{:02X}", b))
        .fold(String::new(), |acc, b| acc + &b);

    println!("{:?}", hex);
    println!();
    std::thread::sleep(Duration::from_secs(5));
}

fn main() {
    let port_name = "/dev/tty.SLAB_USBtoUART";
    let baud_rate = 9600;
    let mut settings: SerialPortSettings = Default::default();

    settings.timeout = Duration::from_millis(10);
    settings.baud_rate = baud_rate.into();

    if let Ok(mut port) = serialport::open_with_settings(&port_name, &settings) {
        let mut buf: [u8; BYTE_NUMS] = [0; BYTE_NUMS];
        let mut serial_buf: Vec<u8> = vec![0; 24];
        let mut cur = 0;

        println!(
            "Ready to receive data on {} at {} rate",
            &port_name, &baud_rate
        );

        loop {
            match port.read(serial_buf.as_mut_slice()) {
                Ok(bytes) => for i in &serial_buf[..bytes] {
                    if cur == 0 && *i != 66u8 {
                        continue;
                    } else if cur == 1 && *i != 77u8 {
                        cur = 0;
                        continue;
                    } else if cur == BYTE_NUMS - 1 {
                        process_bytes(&buf[4..16]);
                        cur = 0;
                    } else {
                        buf[cur] = *i;
                        cur = (cur + 1) % BYTE_NUMS;
                    }
                },
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                Err(e) => eprintln!("{:?}", e),
            }
        }
    }
}
