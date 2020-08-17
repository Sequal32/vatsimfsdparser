mod sniffer;


fn main() {
    let mut client = sniffer::Sniffer::new();
    client.get_user_interface();
    client.start();
    loop {
        if let Some(packet) = client.next() {
            if packet.get_destination_ip().to_string() == "134.209.67.219" {
                let payload = packet.get_payload_as_ascii();
                if payload.trim() == "" {continue;}
                println!("{}", payload);
            }
        }
    }
    
}