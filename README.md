# TeamSpeak 3 Hooker

[![Rust](https://img.shields.io/badge/rust-1.78.0-orange.svg)](https://www.rust-lang.org/)

A proof-of-concept tool for capturing and analyzing TeamSpeak 3 server traffic on the local host. This tool is intended for educational and research purposes only.

## Summary

This implant clings to the UDP traffic of the TeamSpeak3-Server process running on the **same host**. The traffic is then internally parsed and processed. This allows for real-time monitoring and analysis of server activity, including voice communication.

---

## How It Works

The tool operates by identifying the process ID of the TeamSpeak 3 server and then sniffing all UDP traffic associated with that process. The captured packets are parsed to extract meaningful data, such as user information, channel activity, and voice data. The state of the server is managed internally, providing a snapshot of users and channels at any given time.

---

## Key Features

- **Process-Specific Packet Sniffing**: Captures UDP traffic exclusively from the TeamSpeak 3 server process, minimizing noise from other network activity.
- **PCAP Logging**: All captured UDP packets are saved to a `.pcap` file for later analysis and forensic investigation.
- **Real-time State Management**: Maintains an up-to-date representation of the server's state, including users, channels, and their properties.
- **Voice Data Extraction**: Capable of capturing and saving voice data from the server's traffic.
- **ServerQuery Integration**: Uses the TeamSpeak 3 ServerQuery protocol to gather additional information about the server and its clients.

---

## Setup and Configuration

### Prerequisites

- A running TeamSpeak 3 server on the same host.
- Rust and Cargo installed.

### Installation

1.  Clone the repository:
    ```bash
    git clone https://github.com/your-username/ts3_hook_test.git
    cd ts3_hook_test
    ```
2.  Build the project:
    ```bash
    cargo build --release
    ```

### Configuration

Before running the tool, you must configure the ServerQuery credentials in `hooker/src/config.rs`.

-   `SERVERQUERY_USERNAME`: Your ServerQuery username (e.g., `serveradmin`).
-   `SERVERQUERY_PASSWORD`: Your ServerQuery password.
-   `SERVERQUERY_API_KEY`: A valid ServerQuery API key.

---

## Usage

Once the project is built and configured, you can run the tool with:

```bash
cargo run --release
```

The tool will then start sniffing traffic from the TeamSpeak 3 server process. Captured data will be logged to `output.pcap`.

---

## Operational Scenarios and Forensic Benefits

-   **Eavesdropping on Voice Activity**: Users who are alone in a channel may be careless and leak compromising information via voice. This tool can be used to capture that voice data for analysis.
-   **Monitoring Server Activity**: The tool can be used to monitor the activity of a TeamSpeak 3 server in real-time, providing insights into user behavior and server usage patterns.

---

## Documentation

-   **TeamSpeak 3 Protocol**: [ts3protocol.md](https://github.com/ReSpeak/tsdeclarations/blob/e19149d13ec114fd9756bc726e8f86bf47ae9181/ts3protocol.md)
-   **TeamSpeak 3 Packet Definitions**: [ReSpeak/tsdeclarations](https://github.com/ReSpeak/tsdeclarations/tree/master)
-   **pcap Packet Header**: [pcap::PacketHeader](https://docs.rs/pcap/latest/pcap/struct.PacketHeader.html)

---

## Disclaimer

This tool is intended for educational and research purposes only. The author is not responsible for any misuse of this tool. Use at your own risk.
