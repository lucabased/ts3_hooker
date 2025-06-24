use pcap::Packet;

pub fn handle_packet(packet: &Packet) {
    // println!("[HANDLER] Packet handled successfully: {:?}", packet);
    println!("{}", "*".repeat(20));
    // println!("[HANDLER] Packet header: \n {:?}", packet.header);
    println!("\n[HANDLER] Packet data: \n {:?}", packet.data);
}
