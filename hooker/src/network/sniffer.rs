use super::packet_handler;
use crate::state_manager::app_state::AppStateMutex;
use pcap::{Capture, Device, Savefile};
use std::sync::{Arc, Mutex};

pub fn start_sniffing(
    _pid: u32,
    savefile_mutex: Arc<Mutex<Savefile>>,
    app_state: AppStateMutex,
) -> Vec<std::thread::JoinHandle<()>> {
    let devices = Device::list().unwrap();
    let mut handles = vec![];

    for device in devices {
        println!("[DEBUG] Trying device: {:?}", device.name);
        let savefile_mutex = Arc::clone(&savefile_mutex);
        let app_state = Arc::clone(&app_state);
        let handle = std::thread::spawn(move || {
            let mut cap = Capture::from_device(device.clone())
                .unwrap()
                .promisc(true)
                .snaplen(5000)
                .timeout(1)
                .open()
                .unwrap();

            let link_type = cap.get_datalink();

            const TS3_VOICE_PORT: u16 = 9987;
            let filter = format!("udp port {}", TS3_VOICE_PORT);

            println!("[DEBUG] Applying filter: {}", filter);
            if cap.filter(&filter, true).is_err() {
                println!("[DEBUG] Filter not supported on this device");
                return;
            }

            println!(
                "[DEBUG] Starting packet capture loop on device {:?}...",
                device.name
            );
            loop {
                match cap.next_packet() {
                    Ok(packet) => {
                        packet_handler::handle_packet(
                            &packet,
                            &savefile_mutex,
                            &[],
                            link_type,
                            &app_state,
                        );
                    }
                    Err(pcap::Error::TimeoutExpired) => continue, // Ignore timeouts and continue capturing
                    Err(e) => {
                        println!(
                            "[ERROR] Packet capture error on device {:?}: {}",
                            device.name, e
                        );
                        break; // Stop on other errors
                    }
                }
            }
        });
        handles.push(handle);
    }

    handles
}
