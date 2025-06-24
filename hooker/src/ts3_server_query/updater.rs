use crate::state_manager::app_state::AppStateMutex;
use std::time::Duration;
use super::state_updater::update_state;
use ts3_query::QueryClient;

pub async fn start_serverquery_updater(app_state: AppStateMutex) {
    tokio::spawn(async move {
        let mut client = match QueryClient::new("127.0.0.1:10011") {
            Ok(client) => client,
            Err(e) => {
                eprintln!("[ServerQuery] Error creating query client: {}", e);
                return;
            }
        };
        if let Err(e) = client.login("serveradmin", "qDYuZ5OS") {
            eprintln!("[ServerQuery] Error logging in: {}", e);
            return;
        }
        if let Err(e) = client.select_server_by_port(9987) {
            eprintln!("[ServerQuery] Error selecting server: {}", e);
            return;
        }

        loop {
            let app_state_clone = app_state.clone();
            let result = update_state(&mut client, &app_state_clone);

            if let Err(e) = result {
                eprintln!("[ServerQuery] Error updating state: {}", e);
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    });
}
