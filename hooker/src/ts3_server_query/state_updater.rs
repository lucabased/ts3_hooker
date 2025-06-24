use crate::state_manager::app_state::{AppStateMutex, Channel};
use super::client_parser::parse_clients;
use std::collections::HashMap;
use ts3_query::{QueryClient, Ts3Error};

pub fn update_state(
    client: &mut QueryClient,
    app_state: &AppStateMutex,
) -> Result<(), Ts3Error> {
    let response = client.raw_command("clientlist -uid")?;
    let clients = parse_clients(response);
    let channels_result = client.channels();

    let mut new_clients = HashMap::new();
    for c in &clients {
        new_clients.insert(c.id as u16, c.clone());
    }

    let mut new_channels = HashMap::new();
    if let Ok(channels) = channels_result {
        for c in channels {
            new_channels.insert(
                c.cid as u16,
                Channel {
                    id: c.cid as u16,
                    name: c.channel_name,
                },
            );
        }
    }

    let mut state = app_state.lock().unwrap();
    state.clients = new_clients;
    state.channels = new_channels;

    println!("[ServerQuery] State updated");
    Ok(())
}
