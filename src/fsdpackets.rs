use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::util::Frequency;
use serde_json::Value;
use std::fmt::{Formatter, Result};

macro_rules! to_enum {
    ($var:expr) => {
        FromPrimitive::from_u8(force_parse!(u8, $var))
    };
}

macro_rules! force_parse {
    ($to_type:ty, $var:expr) => {
        $var.parse::<$to_type>().unwrap()
    };
}

pub trait Packet {
    fn from_string(fields: &Vec<&str>) -> Self;
}

#[derive(PartialEq, Debug, Clone)]
pub enum TextMessageReceiver {
    Broadcast,
    Wallop,
    ATC,
    PrivateMessage,
    Radio(Frequency),
}
#[derive(FromPrimitive, PartialEq, Debug, Clone)]
pub enum NetworkFacility {
    OBS,
    FSS,
    DEL,
    GND,
    TWR,
    APP,
    CTR,
    Undefined,
}

impl NetworkFacility {
    fn from_string(data: &str) -> Self {
        if data == "" {
            return NetworkFacility::Undefined;
        }
        match to_enum!(data) {
            Some(ok) => ok,
            None => NetworkFacility::Undefined,
        }
    }
}

#[derive(FromPrimitive, PartialEq, Debug, Clone)]
pub enum NetworkRating {
    Undefined,
    OBS,
    S1,
    S2,
    S3,
    C1,
    C2,
    C3,
    I1,
    I2,
    I3,
    SUP,
    ADM,
}

impl std::fmt::Display for NetworkRating {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            NetworkRating::Undefined => write!(f, "{}", "Undefined"),
            NetworkRating::OBS => write!(f, "{}", "OBS"),
            NetworkRating::S1 => write!(f, "{}", "S1"),
            NetworkRating::S2 => write!(f, "{}", "S2"),
            NetworkRating::S3 => write!(f, "{}", "S3"),
            NetworkRating::C1 => write!(f, "{}", "C1"),
            NetworkRating::C2 => write!(f, "{}", "C2"),
            NetworkRating::C3 => write!(f, "{}", "C3"),
            NetworkRating::I1 => write!(f, "{}", "I1"),
            NetworkRating::I2 => write!(f, "{}", "I2"),
            NetworkRating::I3 => write!(f, "{}", "I3"),
            NetworkRating::SUP => write!(f, "{}", "SUP"),
            NetworkRating::ADM => write!(f, "{}", "ADM"),
        }
    }
}

impl NetworkRating {
    fn from_string(data: &str) -> Self {
        if data == "" {
            return NetworkRating::Undefined;
        }
        match to_enum!(data) {
            Some(ok) => ok,
            None => NetworkRating::Undefined,
        }
    }
}

#[derive(FromPrimitive, Debug, PartialEq, Clone)]
pub enum EngineType {
    Piston = 0,
    Jet = 1,
    None = 2,
    Helo = 3,
}

#[derive(FromPrimitive, PartialEq, Debug, Clone)]
pub enum SimulatorType {
    Unknown,
    MSFS95,
    MSFS98,
    MSCFS,
    AS2,
    PS1,
    XPlane,
}

#[derive(FromPrimitive, PartialEq, Debug, Clone)]
pub enum ProtocolRevision {
    Unknown = 0,
    Classic = 9,
    VatsimNoAuth = 10,
    VatsimAuth = 100,
}

// ENUMS //
#[derive(PartialEq, Debug, Clone)]
pub enum NetworkClientType {
    ATC,
    Pilot,
    Undefined,
}

#[derive(PartialEq, Debug, Clone)]
pub enum FlightRules {
    IFR,
    VFR,
    DVFR,
    SVFR,
    Undefined,
}

#[derive(PartialEq, Debug, Clone)]
pub enum SquawkType {
    Standby,
    Charlie,
    Ident,
    Undefined,
}

#[derive(PartialEq, Debug, Clone)]
pub enum ClientQueryType {
    Unknown,
    IsValidATC,
    Capabilities,
    COM1Freq,
    RealName,
    Server,
    ATIS,
    PublicIP,
    INF,
    FlightPlan,
    IPC,
    RequestRelief,
    CancelRequestRelief,
    RequestHelp,
    CancelRequestHelp,
    WhoHas,
    InitiateTrack,
    AcceptHandoff,
    DropTrack,
    SetFinalAltitude,
    SetTempAltitude,
    SetBeaconCode,
    SetScratchpad,
    SetVoiceType,
    AircraftConfiguration,
    NewInfo,
    NewATIS,
}

#[derive(PartialEq, Debug, Clone)]
pub struct TextMessage {
    pub sender: String,
    pub receiver: TextMessageReceiver,
    pub text: String,
}

impl Packet for TextMessage {
    fn from_string(fields: &Vec<&str>) -> Self {
        let receiver_str = fields[1];
        let message = fields[2];

        let receiver = match receiver_str {
            "*" => TextMessageReceiver::Broadcast,
            "*S" => TextMessageReceiver::Wallop,
            "@49999" => TextMessageReceiver::ATC,
            _ => match &receiver_str[0..1] {
                "@" => {
                    TextMessageReceiver::Radio(Frequency::from_packet_string(&receiver_str[1..]))
                }
                _ => TextMessageReceiver::PrivateMessage,
            },
        };

        return TextMessage {
            text: message.to_string(),
            receiver: receiver,
            sender: fields[0].to_string(),
        };
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct NetworkClient {
    pub client_type: NetworkClientType,
    pub callsign: String,
    pub real_name: String,
    pub cid: String,
    pub password: String,
    pub rating: NetworkRating,
    pub protocol_ver: u8,
}

impl Packet for NetworkClient {
    fn from_string(fields: &Vec<&str>) -> Self {
        return NetworkClient::new(fields, NetworkClientType::Undefined);
    }
}

impl NetworkClient {
    pub fn new(fields: &Vec<&str>, client: NetworkClientType) -> Self {
        return match client {
            NetworkClientType::ATC => Self {
                callsign: fields[0].to_string(),
                real_name: fields[2].to_string(),
                cid: fields[3].to_string(),
                password: fields[4].to_string(),
                rating: NetworkRating::from_string(fields[5]),
                protocol_ver: 0,
                client_type: client,
            },
            _ => Self {
                callsign: fields[0].to_string(),
                cid: fields[2].to_string(),
                password: fields[3].to_string(),
                rating: NetworkRating::from_string(fields[4]),
                protocol_ver: force_parse!(u8, fields[5]),
                real_name: fields[7].to_string(),
                client_type: client,
            },
        };
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum SharedStateType {
    Scratchpad,
    BeaconCode,
    VoiceType,
    TempAlt,
    Unknown,
}

#[derive(PartialEq, Debug, Clone)]
pub struct SharedState {
    pub from: String,
    pub to: String,
    pub target: String,
    pub value: String,

    pub shared_type: SharedStateType,
}

impl Packet for SharedState {
    fn from_string(fields: &Vec<&str>) -> Self {
        return Self::new(fields, SharedStateType::Unknown);
    }
}

impl SharedState {
    pub fn new(fields: &Vec<&str>, shared_type: SharedStateType) -> Self {
        return Self {
            from: fields[0].to_string(),
            to: fields[1].to_string(),
            target: fields[4].to_string(),
            value: fields[5].to_string(),
            shared_type: shared_type,
        };
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct FlightStrip {
    pub from: String,
    pub to: String,
    pub target: String,
    pub format_id: String,
    pub annotations: Vec<String>,
}

impl Packet for FlightStrip {
    fn from_string(fields: &Vec<&str>) -> Self {
        let mut annotations: Vec<String> = vec![];

        if fields.len() > 6 {
            for i in 6..fields.len() {
                annotations.push(fields[i].to_string());
            }
        }

        return Self {
            from: fields[0].to_string(),
            to: fields[1].to_string(),
            target: fields[4].to_string(),
            format_id: if fields.len() > 5 {
                fields[5].to_string()
            } else {
                "".to_string()
            },
            annotations: annotations,
        };
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct DeleteClient {
    pub client_type: NetworkClientType,
    pub callsign: String,
    pub cid: String,
}

impl Packet for DeleteClient {
    fn from_string(fields: &Vec<&str>) -> Self {
        return DeleteClient::new(fields, NetworkClientType::Undefined);
    }
}

impl DeleteClient {
    pub fn new(fields: &Vec<&str>, client: NetworkClientType) -> Self {
        return DeleteClient {
            callsign: fields[0].to_string(),
            cid: fields[1].to_string(),
            client_type: client,
        };
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct FlightPlan {
    pub callsign: String,
    pub rule: FlightRules,
    pub equipment: String,
    pub tas: String,
    pub origin: String,
    pub dep_time: String,
    pub actual_dep_time: String,
    pub cruise_alt: String,
    pub dest: String,
    pub hours_enroute: String,
    pub minutes_enroute: String,
    pub fuel_avail_hours: String,
    pub fuel_avail_minutes: String,
    pub alternate: String,
    pub remarks: String,
    pub route: String,

    pub amended_by: Option<String>,
}

impl Packet for FlightPlan {
    fn from_string(fields: &Vec<&str>) -> Self {
        return Self::new(fields, None);
    }
}

impl FlightPlan {
    pub fn new(fields: &Vec<&str>, amended: Option<&str>) -> Self {
        let rule = match fields[2] {
            "I" | "IFR" => FlightRules::IFR,
            "V" | "VFR" => FlightRules::VFR,
            "D" | "DVFR" => FlightRules::DVFR,
            "S" | "SVFR" => FlightRules::SVFR,
            _ => FlightRules::Undefined,
        };

        let amended_by = match amended {
            Some(callsign) => Some(callsign.to_string()),
            None => None,
        };

        return Self {
            callsign: fields[0].to_string(),
            rule: rule,
            equipment: fields[3].to_string(),
            tas: fields[4].to_string(),
            origin: fields[5].to_string(),
            dep_time: fields[6].to_string(),
            actual_dep_time: fields[7].to_string(),
            cruise_alt: fields[8].to_string(),
            dest: fields[9].to_string(),
            hours_enroute: fields[10].to_string(),
            minutes_enroute: fields[11].to_string(),
            fuel_avail_hours: fields[12].to_string(),
            fuel_avail_minutes: fields[13].to_string(),
            alternate: fields[14].to_string(),
            remarks: fields[15].to_string(),
            route: fields[16].to_string(),
            amended_by: amended_by,
        };
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum TransferControlType {
    Received,
    Accepted,
    Cancelled,
    IHaveControl,
    Pointout,
    PushToDepartures,
}

#[derive(PartialEq, Debug, Clone)]
pub struct TransferControl {
    pub from: String,
    pub to: String,
    pub target: String,

    pub transfer_type: TransferControlType,
}

impl Packet for TransferControl {
    fn from_string(fields: &Vec<&str>) -> Self {
        return Self::new(fields, TransferControlType::Received);
    }
}

impl TransferControl {
    pub fn new(fields: &Vec<&str>, transfer_type: TransferControlType) -> Self {
        let target = match transfer_type {
            TransferControlType::Accepted | TransferControlType::Received => fields[2],
            _ => fields[4],
        };

        return Self {
            from: fields[0].to_string(),
            to: fields[1].to_string(),
            target: target.to_string(),

            transfer_type: transfer_type,
        };
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct ATCPosition {
    pub freq: Frequency,
    pub facility: NetworkFacility,
    pub vis_range: u16,
    pub rating: NetworkRating,
    pub lat: f32,
    pub lon: f32,
    pub callsign: String,
}

impl Packet for ATCPosition {
    fn from_string(fields: &Vec<&str>) -> Self {
        return ATCPosition {
            callsign: fields[0].to_string(),
            freq: Frequency::from_packet_string(&fields[1]),
            facility: NetworkFacility::from_string(fields[2]),
            vis_range: force_parse!(u16, fields[3]),
            rating: NetworkRating::from_string(fields[4]),
            lat: fields[5].parse::<f32>().unwrap(),
            lon: fields[6].parse::<f32>().unwrap(),
        };
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct FlightSurfaces {
    pub pitch: f64,
    pub bank: f64,
    pub hdg: f64,
}

impl FlightSurfaces {
    pub fn from_encoded(data: i64) -> FlightSurfaces {
        let mut pitch_dbl = (data >> 22) as f64 / 1024.0 * -360.0;
        let mut bank_dbl = ((data >> 12) & 0x3FF) as f64 / 1024.0 * -360.0;
        let mut hdg_dbl = ((data >> 2) & 0x3FF) as f64 / 1024.0 * 360.0;
        if pitch_dbl > 180.0 {
            pitch_dbl -= 360.0;
        } else if pitch_dbl <= -180.0 {
            pitch_dbl += 360.0;
        }
        if bank_dbl > 180.0 {
            bank_dbl -= 360.0;
        } else if bank_dbl <= -180.0 {
            bank_dbl += 360.0;
        }
        if hdg_dbl < 0.0 {
            hdg_dbl += 360.0;
        } else if hdg_dbl >= 360.0 {
            hdg_dbl -= 360.0;
        }

        return FlightSurfaces {
            hdg: hdg_dbl,
            bank: bank_dbl,
            pitch: pitch_dbl,
        };
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct PilotPosition {
    pub callsign: String,
    pub squawk_code: u16,
    pub squawking: SquawkType,
    pub rating: NetworkRating,
    pub lat: f32,
    pub lon: f32,
    pub true_alt: i32,
    pub pressure_alt: i32,
    pub ground_speed: i32,
    pub pbh: FlightSurfaces,
}

impl Packet for PilotPosition {
    fn from_string(fields: &Vec<&str>) -> Self {
        let squawk_type = match fields[0] {
            "S" => SquawkType::Standby,
            "N" => SquawkType::Charlie,
            "Y" => SquawkType::Ident,
            _ => SquawkType::Undefined,
        };

        let alt = force_parse!(i32, fields[6]);

        return Self {
            callsign: fields[1].to_string(),
            squawk_code: force_parse!(u16, fields[2]),
            squawking: squawk_type,
            rating: NetworkRating::from_string(fields[3]),
            lat: force_parse!(f32, fields[4]),
            lon: force_parse!(f32, fields[5]),
            true_alt: alt,
            pressure_alt: alt + force_parse!(i32, fields[9]),
            ground_speed: force_parse!(i32, fields[7]),
            pbh: FlightSurfaces::from_encoded(force_parse!(i64, fields[8])),
        };
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ClientQueryPayload {
    AcceptHandoff(String, String), // Aircraft Callsign, From ATC
    AircraftConfiguration(Value),
    DropTrack(String),                        // Callsign target
    FlightPlan(String),                       // Callsign of target
    InitiateTrack(String),                    // Callsign
    IsValidATCQuery(Option<String>),          // Callsign target
    IsValidATCResponse(bool, Option<String>), // IsValid, Callsign target
    NewATIS(String),                          // ATIS
    NewInfo(String),                          // Controller info
    RealName(RealNamePayload),
    SetFinalAltitude(String, String), // Callsign, final altitude
    SetBeaconCode(String, String),    // Callsign, Data
    SetScratchpad(String, String),    // Callsign, Data
    SetTempAltitude(String, String),  // Callsign, Altitude
    SetVoiceType(String, String),     // Callsign target, data
    Unknown(Vec<String>),
    WhoHas(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct RealNamePayload {
    pub real_name: String,
    pub facility_name: String,
    pub rating: NetworkRating,
}

impl RealNamePayload {
    fn from_payload(payload: &Vec<String>) -> Self {
        Self {
            real_name: payload[0].to_string(),
            facility_name: payload[1].to_string(),
            rating: to_enum!(payload[2]).unwrap(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ClientQuery {
    pub is_response: bool,
    pub from: String,
    pub to: String,
    pub query_type: ClientQueryType,
    pub payload: ClientQueryPayload,
}

impl Packet for ClientQuery {
    fn from_string(fields: &Vec<&str>) -> Self {
        return Self::new(fields, ClientQueryType::Unknown, false);
    }
}

impl ClientQuery {
    pub fn new(fields: &Vec<&str>, query_type: ClientQueryType, is_response: bool) -> Self {
        let mut payload: Vec<String> = vec![];
        if fields.len() > 3 {
            for i in 3..fields.len() {
                payload.push(fields[i].to_string());
            }
        }
        // Determine payload
        let payload = if payload.len() >= 2 {
            match query_type {
                ClientQueryType::AircraftConfiguration => {
                    ClientQueryPayload::AircraftConfiguration(
                        serde_json::from_str(payload.join(":").as_str()).unwrap(),
                    )
                }
                ClientQueryType::SetBeaconCode => {
                    ClientQueryPayload::SetBeaconCode(payload.remove(0), payload.remove(0))
                }
                ClientQueryType::SetFinalAltitude => {
                    ClientQueryPayload::SetFinalAltitude(payload.remove(0), payload.remove(0))
                }
                ClientQueryType::SetScratchpad => {
                    ClientQueryPayload::SetScratchpad(payload.remove(0), payload.remove(0))
                }
                ClientQueryType::SetTempAltitude => {
                    ClientQueryPayload::SetTempAltitude(payload.remove(0), payload.remove(0))
                }
                ClientQueryType::SetVoiceType => {
                    ClientQueryPayload::SetVoiceType(payload.remove(0), payload.remove(0))
                }
                ClientQueryType::AcceptHandoff => {
                    ClientQueryPayload::AcceptHandoff(payload.remove(0), payload.remove(0))
                }
                _ => ClientQueryPayload::Unknown(payload),
            }
        } else if payload.len() == 1 {
            match query_type {
                ClientQueryType::DropTrack => ClientQueryPayload::DropTrack(payload.remove(0)),
                ClientQueryType::FlightPlan => ClientQueryPayload::FlightPlan(payload.remove(0)),
                ClientQueryType::InitiateTrack => {
                    ClientQueryPayload::InitiateTrack(payload.remove(0))
                }
                ClientQueryType::IsValidATC => match is_response {
                    false => {
                        if payload.len() > 0 {
                            ClientQueryPayload::IsValidATCQuery(Some(payload.remove(0)))
                        } else {
                            ClientQueryPayload::IsValidATCQuery(None)
                        }
                    }
                    true => ClientQueryPayload::IsValidATCResponse(
                        if payload[0] == "Y" { true } else { false },
                        if payload.len() > 1 {
                            Some(payload.remove(0))
                        } else {
                            None
                        },
                    ),
                },
                ClientQueryType::NewATIS => ClientQueryPayload::NewATIS(payload.remove(0)),
                ClientQueryType::NewInfo => ClientQueryPayload::NewInfo(payload.remove(0)),
                ClientQueryType::RealName => match is_response {
                    false => ClientQueryPayload::Unknown(payload),
                    true => ClientQueryPayload::RealName(RealNamePayload::from_payload(&payload)),
                },
                ClientQueryType::WhoHas => ClientQueryPayload::WhoHas(payload.remove(0)),
                _ => ClientQueryPayload::Unknown(payload),
            }
        } else {
            ClientQueryPayload::Unknown(payload)
        };

        return Self {
            is_response: is_response,
            from: fields[0].to_string(),
            to: fields[1].to_string(),
            query_type: query_type,
            payload: payload,
        };
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PlaneInfoRequest {
    pub from: String,
    pub to: String,
}

impl PlaneInfoRequest {
    pub fn new(fields: &Vec<&str>) -> Self {
        Self {
            from: fields[0].to_string(),
            to: fields[1].to_string(),
        }
    }
}

impl Packet for PlaneInfoRequest {
    fn from_string(fields: &Vec<&str>) -> Self {
        Self::new(fields)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum PlaneInfoResponse {
    Legacy {
        csl: String,
        engine_type: EngineType,
    },
    Regular {
        equipment: Option<String>,
        airline: Option<String>,
        livery: Option<String>,
        csl: Option<String>,
    },
}

fn find_value(fields: &Vec<&str>, key: &str) -> Option<String> {
    for &field in fields {
        if field.to_uppercase().starts_with(key) {
            return Some(field[key.len() + 1..].to_string());
        }
    }
    None
}

#[derive(Debug, PartialEq, Clone)]
pub struct PlaneInfo {
    pub from: String,
    pub to: String,
    pub payload: PlaneInfoResponse,
}

impl Packet for PlaneInfo {
    fn from_string(fields: &Vec<&str>) -> Self {
        Self::new(fields)
    }
}

impl PlaneInfo {
    pub fn new(fields: &Vec<&str>) -> Self {
        let is_legacy = fields[3] == "X";

        let payload = match is_legacy {
            true => PlaneInfoResponse::Legacy {
                csl: fields[6].to_string(),
                engine_type: to_enum!(fields[5]).unwrap(),
            },
            false => PlaneInfoResponse::Regular {
                equipment: find_value(fields, "EQUIPMENT"),
                airline: find_value(fields, "AIRLINE"),
                livery: find_value(fields, "LIVERY"),
                csl: find_value(fields, "CSL"),
            },
        };

        Self {
            from: fields[0].to_string(),
            to: fields[1].to_string(),
            payload,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Metar {
    pub is_response: bool,
    pub from: String,
    pub to: String,
    pub payload: String,
}

impl Packet for Metar {
    fn from_string(fields: &Vec<&str>) -> Self {
        return Self::new(fields, false);
    }
}

impl Metar {
    pub fn new(fields: &Vec<&str>, is_response: bool) -> Self {
        Self {
            is_response,
            from: fields[0].to_string(),
            to: fields[1].to_string(),
            payload: fields[3].to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rating_convert() {
        assert_eq!(NetworkRating::from_string(""), NetworkRating::Undefined);
        assert_eq!(NetworkRating::from_string("30"), NetworkRating::Undefined);
        assert_eq!(NetworkRating::from_string("1"), NetworkRating::OBS);
    }

    #[test]
    fn test_facility_convert() {
        assert_eq!(NetworkFacility::from_string(""), NetworkFacility::Undefined);
        assert_eq!(
            NetworkFacility::from_string("30"),
            NetworkFacility::Undefined
        );
        assert_eq!(NetworkFacility::from_string("1"), NetworkFacility::FSS);
    }
}
