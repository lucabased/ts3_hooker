mod process_checker;
mod network;
pub mod ts3_parser;
mod state_manager;
mod ts3_server_query;

use pcap::{Capture};
use std::sync::{Arc, Mutex};
use state_manager::app_state::AppState;

#[tokio::main]
async fn main() {
    let output_file_name = "output.pcap";

    // Create a dummy capture to create the savefile
    let cap = Capture::dead(pcap::Linktype::ETHERNET).unwrap();
    let savefile = cap.savefile(output_file_name).unwrap();
    let savefile_mutex = Arc::new(Mutex::new(savefile));
    let app_state = Arc::new(Mutex::new(AppState::new()));

    ts3_server_query::updater::start_serverquery_updater(Arc::clone(&app_state)).await;

    if let Some(pid) = process_checker::get_ts3_server_pid() {
        println!("TeamSpeak 3 server is running with PID: {}. Starting network sniffer...", pid);
        let _handles = network::sniffer::start_sniffing(pid, Arc::clone(&savefile_mutex), Arc::clone(&app_state));
        
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            let app_state_locked = app_state.lock().unwrap();
            let client_ids: Vec<u16> = app_state_locked.clients.keys().cloned().collect();
            drop(app_state_locked); 
            
            for client_id in client_ids {
                if let Some(voice_data) = ts3_parser::get_last_voice_data(&app_state, client_id) {
                    println!("[Main] Last voice data for client {}: {:?}", client_id, voice_data);
                }
            }
        }
    } else {
        println!("TeamSpeak 3 server is not running.");
    }
}
