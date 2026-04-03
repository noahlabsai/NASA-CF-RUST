# core Flight System (cFS) CFDP Application (CF) — Rust Translation

## Introduction

This is a **Rust translation** of the NASA CFDP application (CF), originally written in C as part of the core Flight System (cFS). The translation was performed using **SENTINEL IDE** by NOAH LABS.

The original CF is a cFS application for providing CFDP (CCSDS File Delivery Protocol) CCSDS 727.0-B-5 compliant services. Its primary function is to provide file receive and transmit functionality to this protocol. It works by mapping CFDP PDUs on and off cFS's software bus.

The original C source code can be found at: https://github.com/nasa/CF

## Translation Details

- **Source Language:** C
- **Target Language:** Rust
- **Tool Used:** Sentinel IDE
- **Translation Approach:** 1:1 structural translation preserving the original architecture
- **Module Organization:** Layered dependency structure (8 layers from foundation to implementation)

### Fully Translated Modules

| Module | Description |
|--------|-------------|
| `cf_crc` | Streaming CRC checksum calculator |
| `cf_timer` | Timer tick/expire management |
| `cf_clist` | Circular doubly-linked list |
| `cf_chunk` | Sparse gap tracking for file segments |
| `cf_codec` | CFDP PDU encoder/decoder |
| `cf_cfdp` | Core CFDP protocol engine |
| `cf_cfdp_r` | Receive-file transaction state machine |
| `cf_cfdp_s` | Send-file transaction state machine |
| `cf_cfdp_dispatch` | PDU dispatch by state/directive |
| `cf_cfdp_sbintf` | Software Bus interface |
| `cf_cmd` | Ground command handlers |
| `cf_utils` | Utility functions |
| `cf_dispatch` | Command message dispatch |
| `cf_app` | Application lifecycle |

## Building

cargo build
cargo test

## SENTINEL IDE 
<img width="2559" height="1385" alt="image" src="https://github.com/user-attachments/assets/f0f7b249-f330-4764-a9e2-f1b9b96a339e" />

<img width="2550" height="1337" alt="image" src="https://github.com/user-attachments/assets/0b767e1e-df45-4313-a6a6-cf4d79228a42" />


