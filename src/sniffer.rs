#![cfg(feature = "sniffer")]
use pnet::datalink;
use pnet::datalink::{MacAddr, NetworkInterface, DataLinkReceiver, Channel};
use pnet::packet::ethernet::{EthernetPacket, EtherTypes};
use pnet::packet::ip::{IpNextHeaderProtocols};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use regex::Regex;
use requests;
use std::collections::{VecDeque, HashSet};
use std::net::Ipv4Addr;
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
                Ok(full_packet) => {
                    Some(full_packet)
                },
                Err(_) => None
            }
        } else {
            panic!("Error reading interface!")
        }
    }
}

const VATSIM_SERVER_FEED: &str = "http://cluster.data.vatsim.net/vatsim-servers.txt";

#[derive(Debug)]
pub enum PacketSource {
    Server(PacketTypes),
    Client(PacketTypes)
}

pub struct Sniffer {
    sniffer: PacketSniffer,
    packet_queue: VecDeque<PacketSource>,
    pub search_ips: HashSet<String>
}

impl Sniffer {
    pub fn new() -> Self {
        return Self {
            sniffer: PacketSniffer::new(),
            search_ips: HashSet::new(),
            packet_queue: VecDeque::new()
        }
    }

    pub fn start(&mut self) {
        self.parse_and_load_server_ips(self.get_server_ips().as_str());
        self.sniffer.start();
    }

    pub fn get_available_interfaces(&self) -> Vec<NetworkInterface> {
        return self.sniffer.get_available_interfaces()
    }

    pub fn set_user_interface(&mut self, interface: &NetworkInterface) {
        self.sniffer.set_user_interface(interface);
    }

    pub fn next(&mut self) -> Option<PacketSource> {
        if self.packet_queue.len() > 0 {return self.packet_queue.pop_front()}

        let packet = self.sniffer.next();
        match packet {
            Some(packet) => {
                let from_server = self.search_ips.contains(&packet.get_source_ip().to_string());
                if from_server || self.search_ips.contains(&packet.get_destination_ip().to_string()) {
                    let text = &packet.get_payload_as_ascii();
                    for payload in text.split("\n") {
                        if let Some(packet) = Parser::parse(payload) {

                            if from_server {
                                self.packet_queue.push_back(PacketSource::Server(packet));
                            }
                            else {
                                self.packet_queue.push_back(PacketSource::Client(packet));
                            }
                        }
                    }
                }
            }
            None => ()
        }

        return self.packet_queue.pop_front();
    }

    fn get_server_ips(&self) -> String {
        let response = requests::get(VATSIM_SERVER_FEED)
            .expect("Could not retrieve VATSIM server list!");

        if !response.status_code().is_success() {panic!("Could not retrieve VATSIM server list!");}

        return response.text().unwrap().to_string();
    }

    fn parse_and_load_server_ips(&mut self, text: &str) {
        let re = Regex::new(r":(\d+.\d+.\d+.\d+):")
            .unwrap();

        for cap in re.captures_iter(text) {
            self.search_ips.insert(cap.get(1).unwrap().as_str().to_string());
        }
        self.search_ips.shrink_to_fit();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;
    #[test]
    fn test_parse_ips() {
        let mut sniffer = Sniffer::new();
        sniffer.parse_and_load_server_ips("!GENERAL:
        VERSION = 8
        RELOAD = 2
        UPDATE = 20200619015411
        ATIS ALLOW MIN = 5
        CONNECTED CLIENTS = 679
        ;
        ;
        !SERVERS:
        AFVDATA:18.130.182.47:Toronto, Canada:AFV Beta Test:1:
        CANADA:165.22.239.218:Toronto, Canada:CANADA:1:
        GERMANY-1:157.230.25.177:Frankfurt, Germany:GERMANY-1:1:
        GERMANY-2:157.230.17.198:Frankfurt, Germany:GERMANY-2:1:
        SINGAPORE:68.183.185.148:Singapore:SINGAPORE:1:
        UK-1:209.97.177.84:London, UK:UK-1:1:
        UK-2:161.35.40.246:London, UK:UK-2:1:
        USA-EAST:134.209.67.219:New York, USA:USA-EAST:1:
        USA-WEST:165.22.163.56:San Francisco, USA:USA-WEST:1:
        ;
        ;   END
        ");
        assert!(sniffer.search_ips.contains(&"165.22.239.218".to_string()));
        assert!(sniffer.search_ips.contains(&"209.97.177.84".to_string()));
        assert!(sniffer.search_ips.contains(&"161.35.40.246".to_string()));
        assert!(sniffer.search_ips.contains(&"18.130.182.47".to_string()));
    }
}