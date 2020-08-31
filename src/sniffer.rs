use pnet::packet::ip::{IpNextHeaderProtocols};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::datalink;
use pnet::datalink::{MacAddr, NetworkInterface, DataLinkReceiver, Channel};
use pnet::packet::ethernet::{EthernetPacket, EtherTypes};
use pnet::packet::tcp::TcpPacket;

use std::{io::{ErrorKind, Error}, net::Ipv4Addr};
use text_io::read;

pub struct EthernetIpv4TCPPacket<'a> {
    payload: &'a [u8],
    ether_packet: EthernetPacket<'a>,
    ipv4_packet: Ipv4Packet<'a>,
    tcp_packet: TcpPacket<'a>
}

impl<'a> EthernetIpv4TCPPacket<'a> {
    pub fn new(packet: &'a [u8]) -> Result<EthernetIpv4TCPPacket<'a>, &'static str> {
        let mut offset = 0;

        let ether_packet = match EthernetPacket::new(packet) {
            Some(packet) => packet,
            None => return Err("Invalid ethernet frame provided!"),
        };

        match ether_packet.get_ethertype() {
            EtherTypes::Ipv4 => {offset += 14},
            _ => return Err("Ethertype not supported.")
        };

        let ipv4_packet = match Ipv4Packet::new(&packet[offset..]) {
            Some(packet) => packet,
            None => return Err("Invalid Ipv4 packet!")
        };

        match ipv4_packet.get_next_level_protocol() {
            IpNextHeaderProtocols::Tcp => (offset += 20),
            _ => return Err("Not TCP packet!")
        }


        let tcp_packet = match TcpPacket::new(&packet[offset..]) {
            Some(packet) => packet,
            None => return Err("Invalid TCP packet!")
        };

        offset += 20;

        Ok(EthernetIpv4TCPPacket {
            payload: &packet[offset..],
            ether_packet: ether_packet,
            ipv4_packet: ipv4_packet,
            tcp_packet: tcp_packet
        })
    }

    pub fn get_source_ip(&self) -> Ipv4Addr {
        return self.ipv4_packet.get_source();
    }

    pub fn get_destination_ip(&self) -> Ipv4Addr {
        return self.ipv4_packet.get_destination();
    }

    pub fn get_source_port(&self) -> u16 {
        return self.tcp_packet.get_source();
    }

    pub fn get_destination_port(&self) -> u16 {
        return self.tcp_packet.get_destination();
    }

    pub fn get_source_mac(&self) -> MacAddr {
        return self.ether_packet.get_source();
    }

    pub fn get_payload(&self) -> &[u8] {
        return self.payload;
    }

    pub fn get_payload_as_ascii(&self) -> String {
        return self.payload.iter().map(|byte| std::char::from_u32(*byte as u32).unwrap().to_string()).collect::<String>();
    }
}

pub struct PacketSniffer {
    rx: Option<Box<dyn DataLinkReceiver>>,
    using_interface: Option<NetworkInterface>,
}

impl PacketSniffer {
    pub fn new() -> PacketSniffer {
        PacketSniffer {
            rx: None,
            using_interface: None
        }
    }

    pub fn get_available_interfaces(&self) -> Vec<NetworkInterface> {
        return datalink::interfaces();
    }

    pub fn set_user_interface(&mut self, interface: &NetworkInterface) {
        self.using_interface = Some(interface.clone());
    }

    pub fn start(&mut self) { // Establish link
        if let None = self.using_interface {panic!("No interface.");}

        self.rx = match datalink::channel(self.using_interface.as_ref().unwrap(), Default::default()) {
            Ok(channel) => {
                match channel {
                    Channel::Ethernet(_, rx) => Some(rx),
                    _ => panic!("Unhandled channel type.")
                }
            },
            Err(_) => panic!("Error opening interface.")
        }
    }

    pub fn next(&mut self) -> Option<EthernetIpv4TCPPacket> {
        if let Ok(packet) = self.rx.as_mut().unwrap().next() {
            return match EthernetIpv4TCPPacket::new(packet) {
                Ok(full_packet) => Some(full_packet),
                Err(_) => None
            }
        } else {
            panic!("Error reading interface!")
        }
    }
}