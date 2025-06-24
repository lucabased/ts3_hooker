
## Prerequisities:
 - Npcap 
 - Cargo



## Docs:
https://github.com/ReSpeak/tsdeclarations/blob/e19149d13ec114fd9756bc726e8f86bf47ae9181/ts3protocol.md




---

## PCAP Packet definition
https://docs.rs/pcap/latest/pcap/struct.PacketHeader.html
**ts**: timeval

The time when the packet was captured
**caplen**: u32

The number of bytes of the packet that are available from the capture
**len**: u32

The length of the packet, in bytes (which might be more than the number of bytes available from the capture, if the length of the packet is larger than the maximum number of bytes to capture)
