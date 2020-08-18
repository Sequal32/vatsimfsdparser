use crate::fsdpackets::{TextMessage, ATCPosition, Packet};

struct Parser {}

#[derive(PartialEq)]
enum PacketTypes {
    TextMessage(TextMessage),
    ATCPosition(ATCPosition)
}

impl Parser {    
    const DELIMETER: &'static str = ":";

    pub fn parse(data: &String) -> PacketTypes {
        let command_prefix = &data[0..1];
        
        match command_prefix {
            "#" => {
                let command = &data[1..3];
                let fields: &Vec<&str> = &data[3..].split(Parser::DELIMETER).collect();
                match command {
                    "TM" => {
                        return PacketTypes::TextMessage(TextMessage::from_string(fields))
                    },
                    _ => panic!("UH OH")
                }
            },
            "%" => {
                let fields: &Vec<&str> = &data[1..].split(Parser::DELIMETER).collect();
                return PacketTypes::ATCPosition(ATCPosition::from_string(fields))
            }
            _ => panic!("UH OH")
        }
    }
}

#[cfg(test)]
mod text_message_tests {
    use super::*;
    use crate::fsdpackets::*;

    macro_rules! test_message {
        ($string: expr, $to_match:path) => {
            let tm = Parser::parse(&$string.to_string());
            match tm {
                PacketTypes::TextMessage(message) => {
                    match message.receiver {
                        $to_match => (),
                        _ => panic!("Not the right receiver type!")
                    }
                }
                _ => panic!("Not the right packet type!")
            }
        };
    }
    #[test]
    fn test_freq_text_message() {
        let tm = Parser::parse(&"#TMNY_CAM_APP:@28120:EK188,turnrightheading310".to_string());
        match tm {
            PacketTypes::TextMessage(message) => {
                assert_eq!(message.sender, "NY_CAM_APP".to_string(), "Sender: {}", message.sender);
                assert_eq!(message.text, "EK188,turnrightheading310");
                match message.receiver {
                    TextMessageReceiver::Radio(freq) => {
                        assert_eq!(freq.text, "128.120")
                    },
                    _ => panic!("Not the right receiver type!")
                }
            }
            _ => panic!("Not the right packet type!")
        }
    }

    #[test]
    fn test_atc_text_message() {
        test_message!("#TMA:@49999:", TextMessageReceiver::ATC);
    }

    #[test]
    fn test_broadcast_text_message() {
        test_message!("#TMA:*:", TextMessageReceiver::Broadcast);
    }

    #[test]
    fn test_wallop_text_message() {
        test_message!("#TMA:*S:", TextMessageReceiver::Wallop);
    }

    #[test]
    fn test_private_text_message() {
        test_message!("#TMA:SWA283:", TextMessageReceiver::PrivateMessage);
    }
}


#[cfg(test)]
mod position_tests {
    use super::*;
    use crate::fsdpackets::*;
    #[test]
    fn test_atc_position() {
        match Parser::parse(&"%BOS_APP:33000:5:150:5:42.35745:-70.98955:0".to_string()) {
            PacketTypes::ATCPosition(pos) => {
                assert_eq!(pos.facility, NetworkFacility::APP);
                assert_eq!(pos.freq.text, "133.000");
                assert_eq!(pos.lat, 42.35745);
                assert_eq!(pos.lon, -70.98955);
                assert_eq!(pos.name, "BOS_APP");
                assert_eq!(pos.rating, NetworkRating::C2);
                assert_eq!(pos.vis_range, 150);
            },
            _ => panic!("Not the right packet type!")
        }
    }
}