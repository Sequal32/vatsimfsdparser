use crate::util;
use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;

use util::{Frequency};

pub trait Packet {
    fn from_string(fields: &Vec<&str>) -> Self;
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum TextMessageReceiver {
    Broadcast,
    Wallop,
    ATC,
    PrivateMessage,
    Radio(Frequency),
}
#[derive(FromPrimitive)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum NetworkFacility {
    OBS,
    FSS,
    DEL,
    GND,
    TWR,
    APP,
    CTR
}

#[derive(FromPrimitive)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum NetworkRating {
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
    ADM
}

#[derive(FromPrimitive)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum SimulatorType {
    Unknown,
    MSFS95,
    MSFS98,
    MSCFS,
    AS2,
    PS1,
    XPlane
}

#[derive(FromPrimitive)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum ProtocolRevision {
    Unknown = 0,
    Classic = 9,
    VatsimNoAuth = 10,
    VatsimAuth = 100
}

// ENUMS //
#[derive(PartialEq)]
#[derive(Debug)]
pub enum NetworkClientType {
    ATC,
    Pilot,
    Undefined
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum FlightRules {
    IFR,
    VFR,
    DVFR,
    SVFR,
    Undefined
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum SquawkType {
    Standby, Charlie, Ident, Undefined
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct TextMessage {
    pub sender: String,
    pub receiver: TextMessageReceiver,
    pub text: String
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
                "@" => TextMessageReceiver::Radio(Frequency::from_packet_string(&receiver_str[1..])),
                _ => TextMessageReceiver::PrivateMessage
            }
        };

        return TextMessage {
            text: message.to_string(),
            receiver: receiver,
            sender: fields[0].to_string()
        }
    }
}

macro_rules! to_enum {
    ($var:expr) => {
        FromPrimitive::from_u8(force_parse!(u8, $var)).unwrap()
    };
}

macro_rules! force_parse {
    ($to_type:ty, $var:expr) => {
        $var.parse::<$to_type>().unwrap()
    };
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct NetworkClient {
    pub client_type: NetworkClientType,
    pub callsign: String,
    pub real_name: String,
    pub cid: String,
    pub rating: NetworkRating,
    pub simulator_type: Option<SimulatorType>
}

impl Packet for NetworkClient {
    fn from_string(fields: &Vec<&str>) -> Self {
        return NetworkClient::new(fields, NetworkClientType::Undefined);
    }
}

impl NetworkClient {
    pub fn new(fields: &Vec<&str>, client: NetworkClientType) -> Self {
        let sim_type = match client {
            NetworkClientType::Pilot => Some(to_enum!(fields[6])),
            _ => None
        };

        return NetworkClient {
            callsign: fields[0].to_string(),
            real_name: fields[2].to_string(),
            cid: fields[3].to_string(),
            rating: to_enum!(fields[4]),
            client_type: client,
            simulator_type: sim_type
        }
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum SharedStateType {
    Scratchpad,
    BeaconCode,
    VoiceType,
    TempAlt,
    Unknown
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct SharedState {
    from: String,
    to: String,
    target: String,
    value: String,
    
    shared_type: SharedStateType
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
            shared_type: shared_type
        }
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct FlightStrip {
    from: String,
    to: String,
    target: String,
    format_id: String,
    annotations: Vec<String>
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
            format_id: if fields.len() > 5 {fields[5].to_string()} else {"".to_string()},
            annotations: annotations
        }
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct DeleteClient {
    pub client_type: NetworkClientType,
    pub callsign: String,
    pub cid: String
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
            client_type: client
        }
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct FlightPlan {
    pub callsign: String,
    pub rule: FlightRules,
    pub aircraft_type: String,
    pub equipment_suffix: Option<String>,
    pub tas: u8,
    pub origin: String,
    pub dep_time: String,
    pub actual_dep_time: String,
    pub cruise_alt: String,
    pub dest: String,
    pub hours_enroute: u8,
    pub minutes_enroute: u8,
    pub fuel_avail_hours: u8,
    pub fuel_avail_minutes: u8,
    pub alternate: String,
    pub remarks: String,
    pub route: String,

    pub amended_by: Option<NetworkClient>
}

impl Packet for FlightPlan {
    fn from_string(fields: &Vec<&str>) -> Self {
        return Self::new(fields, None);
    }
}

impl FlightPlan {
    pub fn new(fields: &Vec<&str>, amended: Option<NetworkClient>) -> Self {
        let rule  = match fields[2] {
            "I" | "IFR" => FlightRules::IFR,
            "V" | "VFR" => FlightRules::VFR,
            "D" | "DVFR" => FlightRules::DVFR,
            "S" | "SVFR" => FlightRules::SVFR,
            _ => FlightRules::Undefined
        };
        return Self {
            callsign: fields[0].to_string(),
            rule: rule,
            aircraft_type: fields[3][0..5].to_string(),
            equipment_suffix: if fields[3].len() > 4 {Some(fields[3][5..].to_string())} else {None},
            tas: force_parse!(u8, fields[4]),
            origin: fields[5].to_string(),
            dep_time: fields[6].to_string(),
            actual_dep_time: fields[7].to_string(),
            cruise_alt: fields[8].to_string(),
            dest: fields[9].to_string(),
            hours_enroute: force_parse!(u8, fields[10]),
            minutes_enroute: force_parse!(u8, fields[11]),
            fuel_avail_hours: force_parse!(u8, fields[12]),
            fuel_avail_minutes: force_parse!(u8, fields[13]),
            alternate: fields[14].to_string(),
            remarks: fields[15].to_string(),
            route: fields[16].to_string(),
            amended_by: amended
        }
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum TransferControlType {
    Received, Accepted, Cancelled, IHaveControl, Pointout, PushToDepartures
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct TransferControl {
    pub from: String,
    pub to: String,
    pub target: String,

    pub transfer_type: TransferControlType
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
            _ => fields[4]
        };

        return Self {
            from: fields[0].to_string(),
            to: fields[1].to_string(),
            target: target.to_string(),

            transfer_type: transfer_type
        }
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct ATCPosition {
    pub freq: Frequency,
    pub facility: NetworkFacility,
    pub vis_range: u8,
    pub rating: NetworkRating,
    pub lat: f32,
    pub lon: f32,
    pub name: String
}

impl Packet for ATCPosition {
    fn from_string(fields: &Vec<&str>) -> Self {
        return ATCPosition {
            name: fields[0].to_string(),
            freq: Frequency::from_packet_string(&fields[1]),
            facility: to_enum!(fields[2]),
            vis_range: force_parse!(u8, fields[3]),
            rating: to_enum!(fields[4]),
            lat: fields[5].parse::<f32>().unwrap(),
            lon: fields[6].parse::<f32>().unwrap()
        }
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct FlightSurfaces {
    pitch: f64,
    bank: f64,
    hdg: f64
}

impl FlightSurfaces {
    fn from_encoded(data: i64) -> FlightSurfaces {
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
            pitch: pitch_dbl
        }
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct PilotPosition {
    pub callsign: String,
    pub transponder_code: u8,
    pub squawking: SquawkType,
    pub rating: NetworkRating,
    pub lat: f32,
    pub lon: f32,
    pub true_alt: i32,
    pub pressure_alt: i32,
    pub ground_speed: i32,
    pub pbh: FlightSurfaces
}

impl Packet for PilotPosition {
    fn from_string(fields: &Vec<&str>) -> Self {
        let squawk_type = match fields[0] {
            "S" => SquawkType::Standby,
            "N" => SquawkType::Charlie,
            "Y" => SquawkType::Ident,
            _ => SquawkType::Undefined
        };

        let alt = force_parse!(i32, fields[6]);

        return Self {
            callsign: fields[1].to_string(),
            transponder_code: force_parse!(u8, fields[2]),
            squawking: squawk_type,
            rating: to_enum!(fields[3]),
            lat: force_parse!(f32, fields[4]),
            lon: force_parse!(f32, fields[5]),
            true_alt: alt,
            pressure_alt: alt + force_parse!(i32, fields[9]),
            ground_speed: force_parse!(i32, fields[7]),
            pbh: FlightSurfaces::from_encoded(force_parse!(i64, fields[8]))
        }
    }
}