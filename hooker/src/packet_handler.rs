use pcap::Packet;

pub fn handle_packet(packet: &Packet) {
    println!("[HANDLER] Packet handled successfully: {:?}", packet);
}
