use fsdparser::Sniffer;
use text_io::read;

fn main() {
    let mut sniffer = Sniffer::new();
    
    //Prompts user for the interface to use
    let interfaces = sniffer.get_available_interfaces();

    for (index, interface) in interfaces.iter().enumerate() {
        println!("{}. {} {}", index, interface.description, interface.ips.iter().map(|ip| ip.to_string() + ", ").collect::<String>());
    }

    println!("Pick an adapter to use: ");
    let i: usize = read!();
    sniffer.set_user_interface(interfaces.get(i).unwrap());
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