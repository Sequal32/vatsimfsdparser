mod sniffer;

use pnet::datalink;
use pnet::packet::ipv4::{Ipv4Packet};
use text_io::read;

fn main() {
    let mut interfaces = datalink::interfaces();
    // Find interface
    for (n, interface) in interfaces.iter().enumerate() {
        for ip in &interface.ips {
            println!("{}. {}", n, ip.to_string());
        }
    }

    println!("Pick the adapter to listen on: ");
    let i: usize = read!();
    let using_interface = interfaces.swap_remove(i);

    match datalink::channel(&using_interface, Default::default()).unwrap() {
        datalink::Channel::Ethernet(_, mut rx) => {
            loop {
                match rx.next() {
                    Ok(packet) => {
                        let new_packet = pnet::packet::ethernet::EthernetPacket::new(packet).unwrap();
                        // println!("{} {}", new_packet.get_source().to_string(), new_packet.get_destination().to_string());
                        // let packet = Ipv4Packet::new(ether_packet.).unwrap();

                        // let typea = format!();
                        let connt_type = packet[12];
                        if connt_type != 8 {continue;}

                        let size = packet[13];

                        let ip_packet = Ipv4Packet::new(&packet[14..]).unwrap();
                        if ip_packet.get_next_level_protocol() != pnet::packet::ip::IpNextHeaderProtocols::Tcp {continue;}
                        // println!("");
                        // println!("{} {} {}", packet.len(), ip_packet.get_total_length(), ip_packet.get_next_level_protocol().to_string());
                        // println!("{}, {}", ip_packet.get_source().to_string(), ip_packet.get_destination().to_string());
                        let tcp_packet = pnet::packet::tcp::TcpPacket::new(&packet[34..]).unwrap();
                        // println!("{} {}", tcp_packet.get_source().to_string(), tcp_packet.get_destination().to_string());

                        if ip_packet.get_source().to_string() != "134.209.67.219" {continue}
                        // println!("");


                        print!("{}", &packet[54..].iter().map(|byte| {std::char::from_u32(*byte as u32).unwrap().to_string().trim().to_string()}).collect::<String>());
                        for byte in &packet[54..] {
                            print!("{} ", std::char::from_u32(*byte as u32).unwrap().to_string());
                        }
                    },
                    Err(e) => panic!("Error reading!")
                }
            }
        }
        _ => panic!("Undefined channel type!")
    }
}