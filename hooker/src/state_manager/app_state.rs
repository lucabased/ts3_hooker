// use crate::ts3_parser::crypto::Ts3Crypto;
use crate::ts3_parser::data_parser::VoiceData;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Client {
    pub id: u16,
    pub channel_id: u16,
    pub nickname: String,
    pub database_id: u64,
    pub unique_id: String,
    // pub crypto: Ts3Crypto,
    pub last_voice_data: Option<VoiceData>,
}

#[derive(Debug, Clone)]
pub struct Channel {
    pub id: u16,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub clients: HashMap<u16, Client>,
    pub channels: HashMap<u16, Channel>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
            channels: HashMap::new(),
        }
    }
}

pub type AppStateMutex = Arc<Mutex<AppState>>;
