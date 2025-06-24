use pcap::{Packet, PacketHeader, Savefile};
use std::sync::{Arc, Mutex};
use std::thread;

/// Spawns a new thread to save a packet to the pcap file.
pub fn save_packet_async(header: PacketHeader, data: Vec<u8>, savefile_mutex: Arc<Mutex<Savefile>>) {
    thread::spawn(move || {
        // Reconstruct a packet on the stack using the owned data.
        let packet_for_saving = Packet {
            header: &header,
            data: &data,
        };
        
        let mut savefile = savefile_mutex.lock().unwrap();
        savefile.write(&packet_for_saving);
    });
}
