use crate::fsdpackets::*;

pub struct Parser {}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum PacketTypes {
    TextMessage(TextMessage),
    ATCPosition(ATCPosition),
    PilotPosition(PilotPosition),
    NetworkClient(NetworkClient),
    DeleteClient(DeleteClient),
    TransferControl(TransferControl),
    SharedState(SharedState),
    FlightStrip(FlightStrip),
    FlightPlan(FlightPlan),
    ClientQuery(ClientQuery),
    Metar(Metar),
}

impl Parser {    
    const DELIMETER: &'static str = ":";

    pub fn parse(data: &str) -> Option<PacketTypes> {
        let data = data.trim().to_string();

        if data.len() == 0 || data.find(":") == None {return None}
        // Make sure first few characters are alphanumeric
        let mut chars = data.chars();
        for _ in 0..3 {
            if !chars.next().unwrap().is_ascii() {return None}
        }

        let command_prefix = &data[0..1];
        match command_prefix {
            "#" | "$" => {
                let command = &data[1..3];
                let fields: &Vec<&str> = &data[3..].split(Parser::DELIMETER).collect();
                match command {
                    "AA" => Some(PacketTypes::NetworkClient(NetworkClient::new(fields, NetworkClientType::ATC))),
                    "DA" => Some(PacketTypes::DeleteClient(DeleteClient::new(fields, NetworkClientType::ATC))),
                    "AP" => Some(PacketTypes::NetworkClient(NetworkClient::new(fields, NetworkClientType::Pilot))),
                    "DP" => Some(PacketTypes::DeleteClient(DeleteClient::new(fields, NetworkClientType::Pilot))),
                    "TM" => Some(PacketTypes::TextMessage(TextMessage::from_string(fields))),
                    "PC" => {
                        match fields[3] {
                            "HC" => Some(PacketTypes::TransferControl(TransferControl::new(fields, TransferControlType::Cancelled))),
                            "ST" => Some(PacketTypes::FlightStrip(FlightStrip::from_string(fields))),
                            "DP" => Some(PacketTypes::TransferControl(TransferControl::new(fields, TransferControlType::PushToDepartures))),
                            "PT" => Some(PacketTypes::TransferControl(TransferControl::new(fields, TransferControlType::Pointout))),
                            "IH" => Some(PacketTypes::TransferControl(TransferControl::new(fields, TransferControlType::IHaveControl))),
                            "SC" => Some(PacketTypes::SharedState(SharedState::new(fields, SharedStateType::Scratchpad))),
                            "BC" => Some(PacketTypes::SharedState(SharedState::new(fields, SharedStateType::BeaconCode))),
                            "VT" => Some(PacketTypes::SharedState(SharedState::new(fields, SharedStateType::VoiceType))),
                            "TA" => Some(PacketTypes::SharedState(SharedState::new(fields, SharedStateType::TempAlt))),
                            _ => None
                        }
                    },
                    "HO" => Some(PacketTypes::TransferControl(TransferControl::new(fields, TransferControlType::Received))),
                    "HA" => Some(PacketTypes::TransferControl(TransferControl::new(fields, TransferControlType::Accepted))),
                    "FP" => Some(PacketTypes::FlightPlan(FlightPlan::from_string(fields))),
                    "AM" => Some(PacketTypes::FlightPlan(FlightPlan::new(fields, Some(fields[17])))),
                    "AR" => Some(PacketTypes::Metar(Metar::new(fields, true))),
                    "AX" => Some(PacketTypes::Metar(Metar::new(fields, false))),
                    "CQ" | "CR" => {
                        let is_response = command == "CR";
                        match fields[2] {
                            "ATC" => Some(PacketTypes::ClientQuery(ClientQuery::new(fields, ClientQueryType::IsValidATC, is_response))),
                            "CAPS" => Some(PacketTypes::ClientQuery(ClientQuery::new(fields, ClientQueryType::Capabilities, is_response))),
                            "C?" => Some(PacketTypes::ClientQuery(ClientQuery::new(fields, ClientQueryType::COM1Freq, is_response))),
                            "RN" => Some(PacketTypes::ClientQuery(ClientQuery::new(fields, ClientQueryType::RealName, is_response))),
                            "SV" => Some(PacketTypes::ClientQuery(ClientQuery::new(fields, ClientQueryType::Server, is_response))),
                            "ATIS" => Some(PacketTypes::ClientQuery(ClientQuery::new(fields, ClientQueryType::ATIS, is_response))),
                            "IP" => Some(PacketTypes::ClientQuery(ClientQuery::new(fields, ClientQueryType::PublicIP, is_response))),
                            "INF" => Some(PacketTypes::ClientQuery(ClientQuery::new(fields, ClientQueryType::INF, is_response))),
                            "FP" => Some(PacketTypes::ClientQuery(ClientQuery::new(fields, ClientQueryType::FlightPlan, is_response))),
                            "IPC" => Some(PacketTypes::ClientQuery(ClientQuery::new(fields, ClientQueryType::IPC, is_response))),
                            "BY" => Some(PacketTypes::ClientQuery(ClientQuery::new(fields, ClientQueryType::RequestRelief, is_response))),
                            "HI" => Some(PacketTypes::ClientQuery(ClientQuery::new(fields, ClientQueryType::CancelRequestRelief, is_response))),
                            "HLP" => Some(PacketTypes::ClientQuery(ClientQuery::new(fields, ClientQueryType::RequestHelp, is_response))),
                            "NOHLP" => Some(PacketTypes::ClientQuery(ClientQuery::new(fields, ClientQueryType::CancelRequestHelp, is_response))),
                            "WH" => Some(PacketTypes::ClientQuery(ClientQuery::new(fields, ClientQueryType::WhoHas, is_response))),
                            "IT" => Some(PacketTypes::ClientQuery(ClientQuery::new(fields, ClientQueryType::InitiateTrack, is_response))),
                            "HT" => Some(PacketTypes::ClientQuery(ClientQuery::new(fields, ClientQueryType::AcceptHandoff, is_response))),
                            "DR" => Some(PacketTypes::ClientQuery(ClientQuery::new(fields, ClientQueryType::DropTrack, is_response))),
                            "FA" => Some(PacketTypes::ClientQuery(ClientQuery::new(fields, ClientQueryType::SetFinalAltitude, is_response))),
                            "TA" => Some(PacketTypes::ClientQuery(ClientQuery::new(fields, ClientQueryType::SetTempAltitude, is_response))),
                            "BC" => Some(PacketTypes::ClientQuery(ClientQuery::new(fields, ClientQueryType::SetBeaconCode, is_response))),
                            "SC" => Some(PacketTypes::ClientQuery(ClientQuery::new(fields, ClientQueryType::SetScratchpad, is_response))),
                            "VT" => Some(PacketTypes::ClientQuery(ClientQuery::new(fields, ClientQueryType::SetVoiceType, is_response))),
                            "ACC" => Some(PacketTypes::ClientQuery(ClientQuery::new(fields, ClientQueryType::AircraftConfiguration, is_response))),
                            "NEWINFO" => Some(PacketTypes::ClientQuery(ClientQuery::new(fields, ClientQueryType::NewInfo, is_response))),
                            "NEWATIS" => Some(PacketTypes::ClientQuery(ClientQuery::new(fields, ClientQueryType::NewATIS, is_response))),
                            _ => Some(PacketTypes::ClientQuery(ClientQuery::new(fields, ClientQueryType::Unknown, is_response)))
                        }
                    }

                    _ => None
                }
            },
            "%" => {
                let fields: &Vec<&str> = &data[1..].split(Parser::DELIMETER).collect();
                Some(PacketTypes::ATCPosition(ATCPosition::from_string(fields)))
            },
            "@" => {
                let fields: &Vec<&str> = &data[1..].split(Parser::DELIMETER).collect();
                Some(PacketTypes::PilotPosition(PilotPosition::from_string(fields)))
            },
            _ => None
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

    #[test]
    fn test_atc_position() {
        match Parser::parse("%BOS_APP:33000:5:150:5:42.35745:-70.98955:0").unwrap() {
            PacketTypes::ATCPosition(pos) => {
                assert_eq!(pos.facility, NetworkFacility::APP);
                assert_eq!(pos.freq.text, "133.000");
                assert_eq!(pos.lat, 42.35745);
                assert_eq!(pos.lon, -70.98955);
                assert_eq!(pos.callsign, "BOS_APP");
                assert_eq!(pos.rating, NetworkRating::C1);
                assert_eq!(pos.vis_range, 150);
            },
            _ => panic!("Not the right packet type!")
        }
    }

    #[test]
    fn test_pilot_position() {
        match Parser::parse("@S:N513PW:4717:1:41.93848:-72.69294:174:0:4282386784:61").unwrap() {
            PacketTypes::PilotPosition(pos) => {
                assert_eq!(pos.callsign, "N513PW");
                assert_eq!(pos.ground_speed, 0);
                assert_eq!(pos.squawking, SquawkType::Standby);
                assert_eq!(pos.squawk_code, 4717);
                assert_eq!(pos.rating, NetworkRating::OBS);
                assert_eq!(pos.lat, 41.93848);
                assert_eq!(pos.lon, -72.69294);
                assert_eq!(pos.pbh.pitch.round(), 1.0);
                assert_eq!(pos.pbh.bank.round(), 0.0);
                assert_eq!(pos.pbh.hdg.round(), 211.0);
                assert_eq!(pos.true_alt, 174);
                assert_eq!(pos.pressure_alt, 235);
            },
            _ => panic!("Not the right packet type!")
        }
    }

    #[test]
    fn test_flight_plan() {
        match Parser::parse("$FPSWA1895:*A:I:B738/L:461:KBNA:1835:1835:35000:KRDU:1:14:3:4:KIAD:GFOSTER85PBN/A1B1C1D1S1S2NAV/RNVD1E2A1REG/N8310CEET/KZTL0012KZDC0044SEL/GPCSRMK/SIMBRIEFAIRAC/2009CHARTSONBOARD:TAZMO3BURMEVXVKPASSALDAN2").unwrap() {
            PacketTypes::FlightPlan(plan) => {
                assert_eq!(plan.callsign, "SWA1895");
                assert_eq!(plan.rule, FlightRules::IFR);
                assert_eq!(plan.equipment, "B738/L");
                assert_eq!(plan.tas, "461");
                assert_eq!(plan.origin, "KBNA");
                assert_eq!(plan.dep_time, "1835");
                assert_eq!(plan.actual_dep_time, "1835");
                assert_eq!(plan.cruise_alt, "35000");
                assert_eq!(plan.dest, "KRDU");
                assert_eq!(plan.hours_enroute, "1");
                assert_eq!(plan.minutes_enroute, "14");
                assert_eq!(plan.fuel_avail_hours, "3");
                assert_eq!(plan.fuel_avail_minutes, "4");
                assert_eq!(plan.alternate, "KIAD");
                assert_eq!(plan.remarks, "GFOSTER85PBN/A1B1C1D1S1S2NAV/RNVD1E2A1REG/N8310CEET/KZTL0012KZDC0044SEL/GPCSRMK/SIMBRIEFAIRAC/2009CHARTSONBOARD");
                assert_eq!(plan.route, "TAZMO3BURMEVXVKPASSALDAN2");
            },
            _ => panic!("Not the right packet type!")
        }
    }

    #[test]
    fn test_metar() {
        match Parser::parse("$AXBOS_GND:SERVER:METAR:KBOS").unwrap() {
            PacketTypes::Metar(metar) => {
                assert_eq!(metar.from, "BOS_GND");
                assert_eq!(metar.to, "SERVER");
                assert_eq!(metar.is_response, false);
                assert_eq!(metar.payload, "KBOS");
            },
            _ => panic!("Not the right packet type!")
        }

        match Parser::parse("$ARSERVER:BOS_GND:METAR:KBOS 180154Z 02011KT 10SM SCT060 OVC250 18/13 A3000 RMK AO2 SLP159 T01780128").unwrap() {
            PacketTypes::Metar(metar) => {
                assert_eq!(metar.to, "BOS_GND");
                assert_eq!(metar.from, "SERVER");
                assert_eq!(metar.is_response, true);
                assert_eq!(metar.payload, "KBOS 180154Z 02011KT 10SM SCT060 OVC250 18/13 A3000 RMK AO2 SLP159 T01780128");
            },
            _ => panic!("Not the right packet type!")
        }
    }
}