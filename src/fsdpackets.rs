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
                "@" => TextMessageReceiver::Radio(Frequency::from_packet_string(&receiver_str[1..].to_string())),
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

macro_rules! to_enum {
    ($var:expr) => {
        FromPrimitive::from_u8($var.parse::<u8>().unwrap()).unwrap()
    };
}

impl Packet for ATCPosition {
    fn from_string(fields: &Vec<&str>) -> Self {
        return ATCPosition {
            name: fields[0].to_string(),
            freq: Frequency::from_packet_string(&fields[1].to_string()),
            facility: to_enum!(fields[2].to_string()),
            vis_range: fields[3].parse::<u8>().unwrap(),
            rating: to_enum!(fields[4].to_string()),
            lat: fields[5].parse::<f32>().unwrap(),
            lon: fields[6].parse::<f32>().unwrap()
        }
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum NetworkClientType {
    ATC,
    Pilot(SimulatorType)
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum FlightRules {
    IFR,
    VFR,
    DVFR,
    SVFR
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct NetworkClient {
    pub client_type: NetworkClientType,
    pub callsign: String,
    pub real_name: String,
    pub cid: String,
    pub password: String,
    pub rating: NetworkRating,
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct DeleteClient {
    pub client_type: NetworkClient,
    pub callsign: String,
    pub cid: String
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct FlightPlan {
    pub callsign: String,
    pub rule: FlightRules,
    pub equipment: String,
    pub tas: u8,
    pub origin: String,
    pub dep_time: String,
    pub actual_dep_time: String,
    pub cruise_alt: String,
    pub dest: String,
    pub hours_enroute: u8,
    pub minutes_enroute: u8,
    pub fuel_avail_hours: String,
    pub fuel_avail_minutes: String,
    pub alternate: String,
    pub remarks: String,
    pub route: String,

    pub amended_by: Option<NetworkClient>
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Handoff {
    pub from: NetworkClient,
    pub to: NetworkClient,
    pub target: NetworkClient
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

#[derive(PartialEq)]
#[derive(Debug)]
pub enum SquawkType {
    Standby, Charlie, Ident
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
    pub pitch: i32,
    pub bank: i32,
    pub heading: i32
}