mod sniffer;
mod fsdpackets;
mod util;
mod parser;

fn main() {
    let mut client = sniffer::Sniffer::new();
    client.get_user_interface();
    client.start();
    loop {
        if let Some(packet) = client.next() {
            if packet.get_source_ip().to_string() == "134.209.67.219" {
                let payloads = packet.get_payload_as_ascii();
                for payload in payloads.split("\n") {
                    if let Ok(data) = parser::Parser::parse(&payload) {
                        println!("{:?}", data);
                    } else {
                        if payload.trim() == "" {continue;}
                        println!("{}", payload);
                    }
                }
                
            }
        }
    }
    
}