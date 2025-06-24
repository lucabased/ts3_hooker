use crate::ts3_parser::ts_declarations::PacketType;
use crate::ts3_parser::{PacketDirection, Ts3Packet};
use byteorder::{BigEndian, ReadBytesExt};
use std::io::Cursor;

#[derive(Debug)]
pub enum PacketData {
    Voice(VoiceData),
    VoiceWhisper(VoiceWhisperData),
    Command(CommandData),
    Ping(PingData),
    Init1(Init1Data),
}

#[derive(Debug, Clone)]
pub struct VoiceData {
    pub client_id: Option<u16>, // Only for ServerToClient
    pub voice_id: u16,
    pub codec: u8,
    pub raw_data: Vec<u8>,
}

#[derive(Debug)]
pub struct VoiceWhisperData {
    pub raw_data: Vec<u8>,
}

#[derive(Debug)]
pub struct CommandData {
    pub command: String,
}

#[derive(Debug)]
pub struct PingData {
    pub raw_data: Vec<u8>,
}

#[derive(Debug)]
pub struct Init1Data {
    pub raw_data: Vec<u8>,
}

pub fn parse_packet_data(
    packet: &Ts3Packet,
    direction: PacketDirection,
) -> Option<PacketData> {
    println!("[DEBUG] Parsing packet of type: {:?}", packet);

    match packet.header.flags.packet_type {
        PacketType::Voice => {
            if direction == PacketDirection::ServerToClient {
                if packet.data.len() < 5 {
                    return None;
                }
                let mut cursor = Cursor::new(&packet.data);
                let client_id = cursor.read_u16::<BigEndian>().unwrap();
                let voice_id = cursor.read_u16::<BigEndian>().unwrap();
                let codec = cursor.read_u8().unwrap();
                let raw_data = packet.data[5..].to_vec();
                Some(PacketData::Voice(VoiceData {
                    client_id: Some(client_id),
                    voice_id,
                    codec,
                    raw_data,
                }))
            } else {
                // ClientToServer
                if packet.data.len() < 3 {
                    return None;
                }
                let mut cursor = Cursor::new(&packet.data);
                let voice_id = cursor.read_u16::<BigEndian>().unwrap();
                let codec = cursor.read_u8().unwrap();
                let raw_data = packet.data[3..].to_vec();
                Some(PacketData::Voice(VoiceData {
                    client_id: None,
                    voice_id,
                    codec,
                    raw_data,
                }))
            }
        }
        PacketType::VoiceWhisper => Some(PacketData::VoiceWhisper(VoiceWhisperData {
            raw_data: packet.data.clone(),
        })),
        PacketType::Command => {
            let command = String::from_utf8_lossy(&packet.data).to_string();
            Some(PacketData::Command(CommandData { command }))
        }
        PacketType::Ping => Some(PacketData::Ping(PingData {
            raw_data: packet.data.clone(),
        })),
        PacketType::Init1 => Some(PacketData::Init1(Init1Data {
            raw_data: packet.data.clone(),
        })),
        _ => None,
    }
}
