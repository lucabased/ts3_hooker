use crate::packet_handler;
use pcap::{Capture, Device};
use std::process::Command;

fn get_ports(pid: u32) -> (Vec<u16>, Vec<u16>) {
    let mut tcp_ports = Vec::new();
    let mut udp_ports = Vec::new();

    // TCP
    let output_tcp = Command::new("netstat")
        .args(&["-ano", "-p", "TCP"])
        .output()
        .expect("failed to execute process");
    let output_str_tcp = String::from_utf8_lossy(&output_tcp.stdout);
    for line in output_str_tcp.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.last() == Some(&pid.to_string().as_str()) {
            if let Some(local_address_str) = parts.get(1) {
                if let Some(port_str) = local_address_str.split(':').last() {
                    if let Ok(port) = port_str.parse::<u16>() {
                        tcp_ports.push(port);
                    }
                }
            }
        }
    }

    // UDP
    let output_udp = Command::new("netstat")
        .args(&["-ano", "-p", "UDP"])
        .output()
        .expect("failed to execute process");
    let output_str_udp = String::from_utf8_lossy(&output_udp.stdout);
    for line in output_str_udp.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.last() == Some(&pid.to_string().as_str()) {
            if let Some(local_address_str) = parts.get(1) {
                if let Some(port_str) = local_address_str.split(':').last() {
                    if let Ok(port) = port_str.parse::<u16>() {
                        udp_ports.push(port);
                    }
                }
            }
        }
    }

    (tcp_ports, udp_ports)
}

pub fn start_sniffing(pid: u32) -> Vec<std::thread::JoinHandle<()>> {
    println!("[DEBUG] Starting sniffer for PID: {}", pid);
    let (tcp_ports, udp_ports) = get_ports(pid);
    if tcp_ports.is_empty() && udp_ports.is_empty() {
        println!("[ERROR] Could not find any ports for PID: {}. Exiting.", pid);
        return vec![];
    }

    let devices = Device::list().unwrap();
    let mut handles = vec![];

    for device in devices {
        println!("[DEBUG] Trying device: {:?}", device.name);
        let tcp_ports = tcp_ports.clone();
        let udp_ports = udp_ports.clone();
        let handle = std::thread::spawn(move || {
            let mut cap = Capture::from_device(device.clone()).unwrap()
                .promisc(true)
                .snaplen(5000)
                .timeout(1)
                .open().unwrap();

            let mut filter_parts = vec![];
            if !tcp_ports.is_empty() {
                filter_parts.push(format!("tcp and ({})", tcp_ports.iter().map(|p| format!("port {}", p)).collect::<Vec<String>>().join(" or ")));
            }
            if !udp_ports.is_empty() {
                filter_parts.push(format!("udp and ({})", udp_ports.iter().map(|p| format!("port {}", p)).collect::<Vec<String>>().join(" or ")));
            }
            let filter = filter_parts.join(" or ");

            println!("[DEBUG] Applying filter: {}", filter);
            if cap.filter(&filter, true).is_err() {
                println!("[DEBUG] Filter not supported on this device");
                return;
            }

            println!("[DEBUG] Starting packet capture loop on device {:?}...", device.name);
            loop {
                match cap.next_packet() {
                    Ok(packet) => {
                        packet_handler::handle_packet(&packet);
                    }
                    Err(pcap::Error::TimeoutExpired) => continue, // Ignore timeouts and continue capturing
                    Err(e) => {
                        println!("[ERROR] Packet capture error on device {:?}: {}", device.name, e);
                        break; // Stop on other errors
                    }
                }
            }
        });
        handles.push(handle);
    }

    handles
}
