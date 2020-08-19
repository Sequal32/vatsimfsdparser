use crate::fsdpackets::*;

struct Parser {}

#[derive(PartialEq)]
enum PacketTypes {
    TextMessage(TextMessage),
    ATCPosition(ATCPosition),
    PilotPosition(PilotPosition),
    NetworkClient(NetworkClient),
    DeleteClient(DeleteClient),
    TransferControl(TransferControl),
    SharedState(SharedState),
    FlightStrip(FlightStrip),
}

impl Parser {    
    const DELIMETER: &'static str = ":";

    pub fn parse(data: &str) -> Result<PacketTypes, &str> {
        let command_prefix = &data[0..1];
        
        match command_prefix {
            "#" | "$" => {
                let command = &data[1..3];
                let fields: &Vec<&str> = &data[3..].split(Parser::DELIMETER).collect();
                match command {
                    "AA" => Ok(PacketTypes::NetworkClient(NetworkClient::new(fields, NetworkClientType::ATC))),
                    "DA" => Ok(PacketTypes::DeleteClient(DeleteClient::new(fields, NetworkClientType::ATC))),
                    "AP" => Ok(PacketTypes::NetworkClient(NetworkClient::new(fields, NetworkClientType::Pilot))),
                    "DP" => Ok(PacketTypes::DeleteClient(DeleteClient::new(fields, NetworkClientType::Pilot))),
                    "TM" => Ok(PacketTypes::TextMessage(TextMessage::from_string(fields))),
                    "#PC" => {
                        match fields[3] {
                            "HC" => Ok(PacketTypes::TransferControl(TransferControl::new(fields, TransferControlType::Cancelled))),
                            "ST" => Ok(PacketTypes::FlightStrip(FlightStrip::from_string(fields))),
                            "DP" => Ok(PacketTypes::TransferControl(TransferControl::new(fields, TransferControlType::PushToDepartures))),
                            "PT" => Ok(PacketTypes::TransferControl(TransferControl::new(fields, TransferControlType::Pointout))),
                            "IH" => Ok(PacketTypes::TransferControl(TransferControl::new(fields, TransferControlType::IHaveControl))),
                            "SC" => Ok(PacketTypes::SharedState(SharedState::new(fields, SharedStateType::Scratchpad))),
                            "BC" => Ok(PacketTypes::SharedState(SharedState::new(fields, SharedStateType::BeaconCode))),
                            "VT" => Ok(PacketTypes::SharedState(SharedState::new(fields, SharedStateType::VoiceType))),
                            "TA" => Ok(PacketTypes::SharedState(SharedState::new(fields, SharedStateType::TempAlt))),
                            _ => Err("Type not handled.")
                        }
                    },
                    "HO" => Ok(PacketTypes::TransferControl(TransferControl::new(fields, TransferControlType::Received))),
                    "HA" => Ok(PacketTypes::TransferControl(TransferControl::new(fields, TransferControlType::Accepted))),

                    _ => Err("Type not handled.")
                }
            },
            "%" => {
                let fields: &Vec<&str> = &data[1..].split(Parser::DELIMETER).collect();
                Ok(PacketTypes::ATCPosition(ATCPosition::from_string(fields)))
            },
            "@" => {
                let fields: &Vec<&str> = &data[1..].split(Parser::DELIMETER).collect();
                Ok(PacketTypes::PilotPosition(PilotPosition::from_string(fields)))
            },
            _ => Err("Type not handled.")
        }
    }
}

#[cfg(test)]
mod text_message_tests {
    use super::*;
    use crate::fsdpackets::*;

    macro_rules! test_message {
        ($string: expr, $to_match:path) => {
            let tm = Parser::parse($string);
            match tm.unwrap() {
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
        let tm = Parser::parse("#TMNY_CAM_APP:@28120:EK188,turnrightheading310");
        match tm.unwrap() {
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
        match Parser::parse(&"%BOS_APP:33000:5:150:5:42.35745:-70.98955:0".to_string()).unwrap() {
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