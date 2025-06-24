
## Prerequisities:
 - Npcap 
 - Cargo


---
## Features
- As a fail-safe, everything UDP packet will be logged to a PCAP file regardless of the processing. This ensures full data integrity and forensic value.
- StateMangement of Users, Channels, UserInfo. 
- Voice capturing to eavesdrop on the dataflow.

---

# Operational scenarios and forensic benefits
- ### Eavesdropping on **Voice-Activity**
    - Users that are **alone within a channel** might be careless and leak compromising information via voice. 
---

## Docs:
https://github.com/ReSpeak/tsdeclarations/blob/e19149d13ec114fd9756bc726e8f86bf47ae9181/ts3protocol.md

---

## TeamSpeak 3 Packet definitions
https://github.com/ReSpeak/tsdeclarations/tree/master


## Team

## PCAP Packet definition
https://docs.rs/pcap/latest/pcap/struct.PacketHeader.html
**ts**: timeval

The time when the packet was captured
**caplen**: u32

The number of bytes of the packet that are available from the capture
**len**: u32

The length of the packet, in bytes (which might be more than the number of bytes available from the capture, if the length of the packet is larger than the maximum number of bytes to capture)
