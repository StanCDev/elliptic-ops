••••• •     •     ••• ••••  ••••• •••  •••        •••  ••••   •••• 
•     •     •      •  •   •   •    •  •          •   • •   • •     
••••  •     •      •  ••••    •    •  •     •••• •   • ••••   •••  
•     •     •      •  •       •    •  •          •   • •         • 
••••• ••••• ••••• ••• •       •   •••  •••        •••  •     ••••  

A modular implementation of Elliptic Curve arithmetic over the **STARK252** and **NIST P-256** prime fields.

## Features
A rust implementation of Elliptic curve addition, doubling, efficient precomputed generator multiplication, and windowed point multiplication.

All functions have supporting tests and benchmarks.

## Supported Curves
| Curve | Field | Use Case |
| --- | --- | --- |
| **STARK252** | $2^{251} + 17 \cdot 2^{192} + 1$ | Starknet, ZK-STARKs, AIR/FRI |
| **NIST P-256** | $2^{256} - 2^{224} + 2^{192} + 2^{96} - 1$ | TLS/SSL, Secure Enclaves, WebAuthn |
