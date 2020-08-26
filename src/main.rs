mod sniffer;
mod fsdpackets;
mod util;
mod parser;
mod vatsimsniffer;

use vatsimsniffer::VatsimSniffer;

fn main() {
    let mut sniffer = VatsimSniffer::new();
    sniffer.get_user_interface();
    sniffer.start();
    loop {
        match sniffer.next() {
            Some(packet) => match packet {
                _ => println!("{:?}", packet)
            },
            None => {}
        }
    }
}