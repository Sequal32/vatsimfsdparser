mod fsdpackets;
mod parser;
mod managers;
mod sniffer;
mod util;

pub use fsdpackets::*;
pub use managers::*;
pub use parser::{Parser, PacketTypes};
use pnet::datalink::NetworkInterface;
use regex::Regex;
use requests;
use sniffer::PacketSniffer;
use std::collections::VecDeque;
use std::net::Ipv4Addr;


const VATSIM_SERVER_FEED: &str = "http://cluster.data.vatsim.net/vatsim-servers.txt";

#[derive(Debug)]
pub enum PacketSource {
    Server(PacketTypes),
    Client(PacketTypes)
}

pub struct Sniffer {
    sniffer: PacketSniffer,
    packet_queue: VecDeque<PacketSource>,
    pub search_ips: Vec<String>
}

impl Sniffer {
    pub fn new() -> Self {
        return Self {
            sniffer: PacketSniffer::new(),
            search_ips: vec![],
            packet_queue: VecDeque::new()
        }
    }

    pub fn is_valid_ip(valid_ips: &Vec<String>, ip: Ipv4Addr) -> bool {
        return valid_ips.contains(&ip.to_string());
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
                let from_server = Self::is_valid_ip(&self.search_ips, packet.get_source_ip());
                if from_server || Self::is_valid_ip(&self.search_ips, packet.get_destination_ip()) {
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
            self.search_ips.push(cap.get(1).unwrap().as_str().to_string());
        }
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
        assert!(Sniffer::is_valid_ip(&sniffer.search_ips, Ipv4Addr::from_str("165.22.239.218").unwrap()));
        assert!(Sniffer::is_valid_ip(&sniffer.search_ips, Ipv4Addr::from_str("209.97.177.84").unwrap()));
        assert!(Sniffer::is_valid_ip(&sniffer.search_ips, Ipv4Addr::from_str("161.35.40.246").unwrap()));
        assert!(Sniffer::is_valid_ip(&sniffer.search_ips, Ipv4Addr::from_str("18.130.182.47").unwrap()));
    }
}