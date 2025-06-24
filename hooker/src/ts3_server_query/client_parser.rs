use crate::state_manager::app_state::Client;
// use crate::ts3_parser::crypto::Ts3Crypto;

pub fn parse_clients(response: Vec<String>) -> Vec<Client> {
    let mut clients = vec![];
    for line in response {
        let mut clid = 0;
        let mut cid = 0;
        let mut client_nickname = "".to_string();
        let mut client_database_id = 0;
        let mut client_unique_identifier = "".to_string();
        for part in line.split(' ') {
            let kv: Vec<&str> = part.split('=').collect();
            if kv.len() != 2 {
                continue;
            }
            match kv[0] {
                "clid" => clid = kv[1].parse().unwrap_or(0),
                "cid" => cid = kv[1].parse().unwrap_or(0),
                "client_nickname" => client_nickname = kv[1].to_string(),
                "client_database_id" => client_database_id = kv[1].parse().unwrap_or(0),
                "client_unique_identifier" => {
                    client_unique_identifier = kv[1].to_string()
                }
                _ => {}
            }
        }
        clients.push(Client {
            id: clid,
            channel_id: cid,
            nickname: client_nickname,
            database_id: client_database_id,
            unique_id: client_unique_identifier,
            // crypto: Ts3Crypto::new(),
            last_voice_data: None,
        });
    }
    clients
}
