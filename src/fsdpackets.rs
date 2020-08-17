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
    Broadcast(),
    Wallop(),
    ATC(),
    Radio(Frequency),
    PrivateMessage()
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

        let receiver = match &receiver_str[0..1] {
            "*" => TextMessageReceiver::Broadcast(),
            "@" => TextMessageReceiver::Radio(Frequency::from_packet_string(&receiver_str[1..].to_string())),
            _ => panic!("Invalid message receiver!")
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
            facility: to_enum!(fields[1]),
            vis_range: fields[2].parse::<u8>().unwrap(),
            rating: FromPrimitive::from_u8(fields[3].parse::<u8>().unwrap()).unwrap(),
            lat: fields[4].parse::<f32>().unwrap(),
            lon: fields[5].parse::<f32>().unwrap()
        }
    }
}