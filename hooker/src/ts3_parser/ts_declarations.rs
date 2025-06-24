use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PacketType {
    Voice = 0x00,
    VoiceWhisper = 0x01,
    Command = 0x02,
    CommandLow = 0x03,
    Ping = 0x04,
    Pong = 0x05,
    Ack = 0x06,
    AckLow = 0x07,
    Init1 = 0x08,
    Unknown = 0xFF,
}

impl From<u8> for PacketType {
    fn from(val: u8) -> Self {
        match val {
            0x00 => PacketType::Voice,
            0x01 => PacketType::VoiceWhisper,
            0x02 => PacketType::Command,
            0x03 => PacketType::CommandLow,
            0x04 => PacketType::Ping,
            0x05 => PacketType::Pong,
            0x06 => PacketType::Ack,
            0x07 => PacketType::AckLow,
            0x08 => PacketType::Init1,
            _ => PacketType::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PacketFlags {
    pub unencrypted: bool,
    pub compressed: bool,
    pub new_protocol: bool,
    pub fragmented: bool,
    pub packet_type: PacketType,
}

impl PacketFlags {
    pub fn from_byte(byte: u8) -> Self {
        Self {
            unencrypted: (byte & 0x80) != 0,
            compressed: (byte & 0x40) != 0,
            new_protocol: (byte & 0x20) != 0,
            fragmented: (byte & 0x10) != 0,
            packet_type: PacketType::from(byte & 0x0F),
        }
    }

    pub fn to_byte(&self) -> u8 {
        let mut byte = self.packet_type as u8;
        if self.unencrypted {
            byte |= 0x80;
        }
        if self.compressed {
            byte |= 0x40;
        }
        if self.new_protocol {
            byte |= 0x20;
        }
        if self.fragmented {
            byte |= 0x10;
        }
        byte
    }
}

#[derive(Debug, Clone)]
pub struct ClientToServerPacket {
    pub mac: [u8; 8],
    pub packet_id: u16,
    pub client_id: u16,
    pub flags: PacketFlags,
    pub data: Vec<u8>,
}

impl ClientToServerPacket {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor = Cursor::new(bytes);
        let mut mac = [0u8; 8];
        cursor.read_exact(&mut mac)?;
        let packet_id = cursor.read_u16::<BigEndian>()?;
        let client_id = cursor.read_u16::<BigEndian>()?;
        let flags_byte = cursor.read_u8()?;
        let flags = PacketFlags::from_byte(flags_byte);
        let mut data = Vec::new();
        cursor.read_to_end(&mut data)?;
        Ok(Self {
            mac,
            packet_id,
            client_id,
            flags,
            data,
        })
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes = Vec::new();
        bytes.write_all(&self.mac)?;
        bytes.write_u16::<BigEndian>(self.packet_id)?;
        bytes.write_u16::<BigEndian>(self.client_id)?;
        bytes.write_u8(self.flags.to_byte())?;
        bytes.write_all(&self.data)?;
        Ok(bytes)
    }
}

#[derive(Debug, Clone)]
pub struct ServerToClientPacket {
    pub mac: [u8; 8],
    pub packet_id: u16,
    pub flags: PacketFlags,
    pub data: Vec<u8>,
}

impl ServerToClientPacket {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor = Cursor::new(bytes);
        let mut mac = [0u8; 8];
        cursor.read_exact(&mut mac)?;
        let packet_id = cursor.read_u16::<BigEndian>()?;
        let flags_byte = cursor.read_u8()?;
        let flags = PacketFlags::from_byte(flags_byte);
        let mut data = Vec::new();
        cursor.read_to_end(&mut data)?;
        Ok(Self {
            mac,
            packet_id,
            flags,
            data,
        })
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes = Vec::new();
        bytes.write_all(&self.mac)?;
        bytes.write_u16::<BigEndian>(self.packet_id)?;
        bytes.write_u8(self.flags.to_byte())?;
        bytes.write_all(&self.data)?;
        Ok(bytes)
    }
}

#[derive(Debug, Clone)]
pub struct VoicePacketData {
    pub voice_id: u16,
    pub codec: u8,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct VoiceWhisperPacketData {
    pub voice_id: u16,
    pub codec: u8,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct AckPacketData {
    pub packet_id: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum GroupWhisperType {
    ServerGroup = 0,
    ChannelGroup = 1,
    ChannelCommander = 2,
    AllClients = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum GroupWhisperTarget {
    AllChannels = 0,
    CurrentChannel = 1,
    ParentChannel = 2,
    AllParentChannel = 3,
    ChannelFamily = 4,
    CompleteChannelFamily = 5,
    Subchannels = 6,
}

#[derive(Debug, Clone)]
pub struct InitPacket0 {
    pub version: u32,
    pub step: u8,
    pub timestamp: u32,
    pub random: [u8; 4],
}

impl InitPacket0 {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor = Cursor::new(bytes);
        let version = cursor.read_u32::<BigEndian>()?;
        let step = cursor.read_u8()?;
        let timestamp = cursor.read_u32::<BigEndian>()?;
        let mut random = [0u8; 4];
        cursor.read_exact(&mut random)?;
        Ok(Self {
            version,
            step,
            timestamp,
            random,
        })
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes = Vec::new();
        bytes.write_u32::<BigEndian>(self.version)?;
        bytes.write_u8(self.step)?;
        bytes.write_u32::<BigEndian>(self.timestamp)?;
        bytes.write_all(&self.random)?;
        Ok(bytes)
    }
}

#[derive(Debug, Clone)]
pub struct InitPacket1 {
    pub step: u8,
    pub server_stuff: [u8; 16],
    pub random_reversed: [u8; 4],
}

impl InitPacket1 {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor = Cursor::new(bytes);
        let step = cursor.read_u8()?;
        let mut server_stuff = [0u8; 16];
        cursor.read_exact(&mut server_stuff)?;
        let mut random_reversed = [0u8; 4];
        cursor.read_exact(&mut random_reversed)?;
        Ok(Self {
            step,
            server_stuff,
            random_reversed,
        })
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes = Vec::new();
        bytes.write_u8(self.step)?;
        bytes.write_all(&self.server_stuff)?;
        bytes.write_all(&self.random_reversed)?;
        Ok(bytes)
    }
}

#[derive(Debug, Clone)]
pub struct InitPacket2 {
    pub version: u32,
    pub step: u8,
    pub server_stuff: [u8; 16],
    pub random_reversed: [u8; 4],
}

impl InitPacket2 {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor = Cursor::new(bytes);
        let version = cursor.read_u32::<BigEndian>()?;
        let step = cursor.read_u8()?;
        let mut server_stuff = [0u8; 16];
        cursor.read_exact(&mut server_stuff)?;
        let mut random_reversed = [0u8; 4];
        cursor.read_exact(&mut random_reversed)?;
        Ok(Self {
            version,
            step,
            server_stuff,
            random_reversed,
        })
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes = Vec::new();
        bytes.write_u32::<BigEndian>(self.version)?;
        bytes.write_u8(self.step)?;
        bytes.write_all(&self.server_stuff)?;
        bytes.write_all(&self.random_reversed)?;
        Ok(bytes)
    }
}

#[derive(Debug, Clone)]
pub struct InitPacket3 {
    pub step: u8,
    pub x: [u8; 64],
    pub n: [u8; 64],
    pub level: u32,
    pub server_stuff: [u8; 100],
}

impl InitPacket3 {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor = Cursor::new(bytes);
        let step = cursor.read_u8()?;
        let mut x = [0u8; 64];
        cursor.read_exact(&mut x)?;
        let mut n = [0u8; 64];
        cursor.read_exact(&mut n)?;
        let level = cursor.read_u32::<BigEndian>()?;
        let mut server_stuff = [0u8; 100];
        cursor.read_exact(&mut server_stuff)?;
        Ok(Self {
            step,
            x,
            n,
            level,
            server_stuff,
        })
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes = Vec::new();
        bytes.write_u8(self.step)?;
        bytes.write_all(&self.x)?;
        bytes.write_all(&self.n)?;
        bytes.write_u32::<BigEndian>(self.level)?;
        bytes.write_all(&self.server_stuff)?;
        Ok(bytes)
    }
}

#[derive(Debug, Clone)]
pub struct InitPacket4 {
    pub version: u32,
    pub step: u8,
    pub x: [u8; 64],
    pub n: [u8; 64],
    pub level: u32,
    pub server_stuff: [u8; 100],
    pub y: [u8; 64],
    pub client_init_iv_data: Vec<u8>,
}

impl InitPacket4 {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor = Cursor::new(bytes);
        let version = cursor.read_u32::<BigEndian>()?;
        let step = cursor.read_u8()?;
        let mut x = [0u8; 64];
        cursor.read_exact(&mut x)?;
        let mut n = [0u8; 64];
        cursor.read_exact(&mut n)?;
        let level = cursor.read_u32::<BigEndian>()?;
        let mut server_stuff = [0u8; 100];
        cursor.read_exact(&mut server_stuff)?;
        let mut y = [0u8; 64];
        cursor.read_exact(&mut y)?;
        let mut client_init_iv_data = Vec::new();
        cursor.read_to_end(&mut client_init_iv_data)?;
        Ok(Self {
            version,
            step,
            x,
            n,
            level,
            server_stuff,
            y,
            client_init_iv_data,
        })
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes = Vec::new();
        bytes.write_u32::<BigEndian>(self.version)?;
        bytes.write_u8(self.step)?;
        bytes.write_all(&self.x)?;
        bytes.write_all(&self.n)?;
        bytes.write_u32::<BigEndian>(self.level)?;
        bytes.write_all(&self.server_stuff)?;
        bytes.write_all(&self.y)?;
        bytes.write_all(&self.client_init_iv_data)?;
        Ok(bytes)
    }
}
