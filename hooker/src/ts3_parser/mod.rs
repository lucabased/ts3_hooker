use byteorder::{BigEndian, ReadBytesExt};
use std::io::{Cursor, Read};

pub mod crypto;
pub mod data_parser;
pub mod ts_declarations;
use crate::state_manager::app_state::AppStateMutex;
use data_parser::{PacketData, VoiceData};
use ts_declarations::PacketFlags;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PacketDirection {
    ClientToServer,
    ServerToClient,
}


#[derive(Debug)]
pub struct Header {
    pub mac: [u8; 8],
    pub packet_id: u16,
    pub client_id: Option<u16>, // Present in Client -> Server packets
    pub flags: PacketFlags,
}

#[derive(Debug)]
pub struct Ts3Packet {
    pub header: Header,
    pub data: Vec<u8>,
    pub packet_data: Option<PacketData>,
}

/// Parses a TeamSpeak 3 packet from a raw UDP payload.
pub fn parse_ts3_packet(data: &[u8], direction: PacketDirection) -> Option<Ts3Packet> {
    let (header, data) = match direction {
        PacketDirection::ClientToServer => {
            println!(
                "[DEBUG] Parsing ClientToServer packet with data length: {}",
                data.len()
            );

            if data.len() < 13 {
                return None;
            }
            let mut rdr = Cursor::new(data);
            let mut mac = [0u8; 8];
            if rdr.read_exact(&mut mac).is_err() {
                return None;
            }
            let packet_id = rdr.read_u16::<BigEndian>().ok()?;
            let client_id = rdr.read_u16::<BigEndian>().ok()?;
            let pt_byte = rdr.read_u8().ok()?;

            let flags = PacketFlags::from_byte(pt_byte);

            let header = Header {
                mac,
                packet_id,
                client_id: Some(client_id),
                flags,
            };
            (header, &data[13..])
        }
        PacketDirection::ServerToClient => {
            println!(
                "[DEBUG] Parsing ServerToClient packet with data length: {}",
                data.len()
            );
            if data.len() < 11 {
                return None;
            }
            let mut rdr = Cursor::new(data);
            let mut mac = [0u8; 8];
            if rdr.read_exact(&mut mac).is_err() {
                return None;
            }
            let packet_id = rdr.read_u16::<BigEndian>().ok()?;
            let pt_byte = rdr.read_u8().ok()?;

            let flags = PacketFlags::from_byte(pt_byte);

            let header = Header {
                mac,
                packet_id,
                client_id: None,
                flags,
            };
            (header, &data[11..])
        }
    };

    let mut packet = Ts3Packet {
        header,
        data: data.to_vec(),
        packet_data: None,
    };

    packet.packet_data = data_parser::parse_packet_data(&packet, direction);

    Some(packet)
}

pub fn get_last_voice_data(
    app_state: &AppStateMutex,
    client_id: u16,
) -> Option<VoiceData> {
    let app_state = app_state.lock().unwrap();
    app_state
        .clients
        .get(&client_id)
        .and_then(|client| client.last_voice_data.clone())
}
