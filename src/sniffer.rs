#![cfg(feature = "sniffer")]
use crate::parser::{PacketTypes, Parser};
use pnet::datalink;
use pnet::datalink::{Channel, DataLinkReceiver, MacAddr, NetworkInterface};
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use serde::Deserialize;
use std::collections::{HashSet, VecDeque};
use std::net::Ipv4Addr;

pub struct EthernetIpv4TCPPacket<'a> {
    payload: &'a [u8],
    ether_packet: EthernetPacket<'a>,
    ipv4_packet: Ipv4Packet<'a>,
    tcp_packet: TcpPacket<'a>,
}

impl<'a> EthernetIpv4TCPPacket<'a> {
    pub fn new(packet: &'a [u8]) -> Result<EthernetIpv4TCPPacket<'a>, &'static str> {
        let mut offset = 0;

        let ether_packet = match EthernetPacket::new(packet) {
            Some(packet) => packet,
            None => return Err("Invalid ethernet frame provided!"),
        };

        match ether_packet.get_ethertype() {
            EtherTypes::Ipv4 => offset += 14,
            _ => return Err("Ethertype not supported."),
        };

        let ipv4_packet = match Ipv4Packet::new(&packet[offset..]) {
            Some(packet) => packet,
            None => return Err("Invalid Ipv4 packet!"),
        };

        match ipv4_packet.get_next_level_protocol() {
            IpNextHeaderProtocols::Tcp => (offset += 20),
            _ => return Err("Not TCP packet!"),
        }

        let tcp_packet = match TcpPacket::new(&packet[offset..]) {
            Some(packet) => packet,
            None => return Err("Invalid TCP packet!"),
        };

        offset += 20;

        Ok(EthernetIpv4TCPPacket {
            payload: &packet[offset..],
            ether_packet: ether_packet,
            ipv4_packet: ipv4_packet,
            tcp_packet: tcp_packet,
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
        return self
            .payload
            .iter()
            .map(|byte| std::char::from_u32(*byte as u32).unwrap().to_string())
            .collect::<String>();
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
            using_interface: None,
        }
    }

    pub fn get_available_interfaces(&self) -> Vec<NetworkInterface> {
        return datalink::interfaces();
    }

    pub fn set_user_interface(&mut self, interface: &NetworkInterface) {
        self.using_interface = Some(interface.clone());
    }

    pub fn start(&mut self) {
        // Establish link
        if let None = self.using_interface {
            panic!("No interface.");
        }

        self.rx =
            match datalink::channel(self.using_interface.as_ref().unwrap(), Default::default()) {
                Ok(channel) => match channel {
                    Channel::Ethernet(_, rx) => Some(rx),
                    _ => panic!("Unhandled channel type."),
                },
                Err(_) => panic!("Error opening interface."),
            }
    }

    pub fn next(&mut self) -> Option<EthernetIpv4TCPPacket> {
        if let Ok(packet) = self.rx.as_mut().unwrap().next() {
            return match EthernetIpv4TCPPacket::new(packet) {
                Ok(full_packet) => Some(full_packet),
                Err(_) => None,
            };
        } else {
            panic!("Error reading interface!")
        }
    }
}

const VATSIM_SERVER_FEED: &str = "https://data.vatsim.net/v3/vatsim-data.json";

#[derive(Deserialize)]
struct DataFeed {
    servers: Vec<Server>,
}

#[derive(Deserialize)]
struct Server {
    hostname_or_ip: String,
}

#[derive(Debug)]
pub enum PacketSource {
    Server(PacketTypes),
    Client(PacketTypes),
}

pub struct Sniffer {
    sniffer: PacketSniffer,
    packet_queue: VecDeque<PacketSource>,
    pub search_ips: HashSet<String>,
}

impl Sniffer {
    pub fn new() -> Self {
        return Self {
            sniffer: PacketSniffer::new(),
            search_ips: HashSet::new(),
            packet_queue: VecDeque::new(),
        };
    }

    pub fn start(&mut self) {
        self.load_server_ips();
        self.sniffer.start();
    }

    pub fn get_available_interfaces(&self) -> Vec<NetworkInterface> {
        return self.sniffer.get_available_interfaces();
    }

    pub fn set_user_interface(&mut self, interface: &NetworkInterface) {
        self.sniffer.set_user_interface(interface);
    }

    pub fn next(&mut self) -> Option<PacketSource> {
        if self.packet_queue.len() > 0 {
            return self.packet_queue.pop_front();
        }

        let packet = self.sniffer.next();
        match packet {
            Some(packet) => {
                let from_server = self
                    .search_ips
                    .contains(&packet.get_source_ip().to_string());
                if from_server
                    || self
                        .search_ips
                        .contains(&packet.get_destination_ip().to_string())
                {
                    let text = &packet.get_payload_as_ascii();
                    for payload in text.split("\n") {
                        if let Some(packet) = Parser::parse(payload) {
                            if from_server {
                                self.packet_queue.push_back(PacketSource::Server(packet));
                            } else {
                                self.packet_queue.push_back(PacketSource::Client(packet));
                            }
                        }
                    }
                }
            }
            None => (),
        }

        return self.packet_queue.pop_front();
    }

    fn get_servers(&self) -> Vec<Server> {
        let response =
            requests::get(VATSIM_SERVER_FEED).expect("Could not retrieve VATSIM server list!");

        if !response.status_code().is_success() {
            panic!("Could not retrieve VATSIM server list!");
        }

        let data = response
            .text()
            .and_then(|x| serde_json::from_str::<DataFeed>(x).ok())
            .expect("Could not deserialize VATSIM server list!");

        return data.servers;
    }

    fn load_server_ips(&mut self) {
        let servers = self.get_servers();
        self.search_ips
            .extend(servers.into_iter().map(|s| s.hostname_or_ip));
    }
}
