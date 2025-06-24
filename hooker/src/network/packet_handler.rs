use crate::state_manager::app_state::AppStateMutex;
use colored::Colorize;
use pcap::{Linktype, Packet, Savefile};
use std::sync::{Arc, Mutex};

use super::pcap_saver::save_packet_async;
use crate::ts3_parser::ts_declarations::PacketType;
use crate::ts3_parser::{parse_ts3_packet, PacketDirection};
use super::udp_parser::parse_udp_packet;

/// Handles an incoming packet by creating owned copies of its data and
/// passing them to the asynchronous save function.
pub fn handle_packet(
    packet: &Packet,
    savefile_mutex: &Arc<Mutex<Savefile>>,
    _local_udp_ports: &[u16],
    link_type: Linktype,
    app_state: &AppStateMutex,
) {
    let owned_header = *packet.header;
    let owned_data = packet.data.to_vec();
    let savefile_mutex_clone = Arc::clone(savefile_mutex);

    save_packet_async(owned_header, owned_data.clone(), savefile_mutex_clone);

    println!("{}", "*".repeat(50));
    // Debug output to indicate packet handling
    println!("[DEBUG] Packet handled: {} bytes", packet.header.len);

    if let Some(udp_packet) = parse_udp_packet(&owned_data, link_type) {
        const TS3_VOICE_PORT: u16 = 9987;
        let direction = if udp_packet.dest_port == TS3_VOICE_PORT {
            PacketDirection::ClientToServer
        } else if udp_packet.src_port == TS3_VOICE_PORT {
            PacketDirection::ServerToClient
        } else {
            // If the packet is not coming from or going to the default TS3 voice port,
            // we can't determine its direction. We'll skip it.
            return;
        };

        let mut app_state_locked = app_state.lock().unwrap();
        if let Some(ts3_packet) = parse_ts3_packet(&udp_packet.payload, direction) {
            let client_id = if direction == PacketDirection::ClientToServer {
                ts3_packet.header.client_id
            } else if let Some(crate::ts3_parser::data_parser::PacketData::Voice(ref voice_data)) =
                ts3_packet.packet_data
            {
                voice_data.client_id
            } else {
                None
            };

            if let Some(client_id) = client_id {
                println!("[DEBUG] Client ID: {}", client_id);
                if !app_state_locked.clients.contains_key(&client_id) {
                    let new_client = crate::state_manager::app_state::Client {
                        id: client_id,
                        channel_id: 0, // Default channel_id
                        nickname: "Unknown".to_string(),
                        database_id: 0,
                        unique_id: "".to_string(),
                        last_voice_data: None,
                    };
                    app_state_locked.clients.insert(client_id, new_client);
                }
                if let Some(client) = app_state_locked.clients.get_mut(&client_id) {
                    if ts3_packet.header.flags.packet_type == PacketType::Voice {
                        if let Some(crate::ts3_parser::data_parser::PacketData::Voice(
                            voice_data,
                        )) = ts3_packet.packet_data
                        {
                            let channel_id = client.channel_id;
                            println!(
                                "{}",
                                format!(
                                    "[TS3] Voice packet: ClientID={}, ChannelID={}, VoiceID={}, Codec={}, DataLen={}",
                                    client_id,
                                    channel_id,
                                    voice_data.voice_id,
                                    voice_data.codec,
                                    voice_data.raw_data.len()
                                )
                                .green()
                            );
                            client.last_voice_data = Some(voice_data);
                        }
                    } else if ts3_packet.header.flags.packet_type != PacketType::VoiceWhisper
                        && ts3_packet.header.flags.packet_type != PacketType::Ping
                        && ts3_packet.header.flags.packet_type != PacketType::Pong
                        && ts3_packet.header.flags.packet_type != PacketType::Ack
                        && ts3_packet.header.flags.packet_type != PacketType::AckLow
                        && ts3_packet.header.flags.packet_type != PacketType::Command
                        && ts3_packet.header.flags.packet_type != PacketType::CommandLow
                    {
                        println!("[TS3] Parsed packet: {:?}", ts3_packet);
                    }
                }
            }
        }
    }
}
