#[derive(Debug)]
pub struct UdpPacket {
    pub src_port: u16,
    pub dest_port: u16,
    pub length: u16,
    pub payload: Vec<u8>,
}

/// Parses the UDP segment from the packet data.
use pcap::Linktype;

pub fn parse_udp_packet(data: &[u8], link_type: Linktype) -> Option<UdpPacket> {
    let eth_header_len = match link_type {
        Linktype::ETHERNET => 14,
        Linktype::NULL => 4,
        _ => {
            println!("[DEBUG] Unsupported link type: {:?}", link_type);
            return None;
        }
    };

    if data.len() < eth_header_len {
        println!("[DEBUG] Packet too small for Ethernet header");
        return None;
    }

    // IP header starts after Ethernet header
    let ip_header_start = eth_header_len;
    let ip_header = &data[ip_header_start..];

    // IP header length is in the lower 4 bits of the first byte, in 32-bit words
    let ip_header_len = (ip_header[0] & 0x0F) as usize * 4;
    if ip_header.len() < ip_header_len {
        println!("[DEBUG] Packet too small for IP header");
        return None;
    }

    // UDP segment starts after IP header
    let udp_segment_start = ip_header_start + ip_header_len;
    let udp_segment = &data[udp_segment_start..];

    if udp_segment.len() < 8 {
        println!("[DEBUG] Packet too small for UDP header");
        return None;
    }

    let src_port = u16::from_be_bytes([udp_segment[0], udp_segment[1]]);
    let dest_port = u16::from_be_bytes([udp_segment[2], udp_segment[3]]);
    let length = u16::from_be_bytes([udp_segment[4], udp_segment[5]]);
    let payload = udp_segment[8..].to_vec();

    Some(UdpPacket {
        src_port,
        dest_port,
        length,
        payload,
    })
}
