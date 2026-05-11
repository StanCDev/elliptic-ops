```
 _____ _ _ _       _   _            ___
| ____| | (_)_ __ | |_(_) ___      / _ \ _ __  ___
|  _| | | | | '_ \| __| |/ __|____| | | | '_ \/ __|
| |___| | | | |_) | |_| | (_|_____| |_| | |_) \__ \
|_____|_|_|_| .__/ \__|_|\___|     \___/| .__/|___/
            |_|                         |_|
```
A modular Rust implementation of Elliptic Curve arithmetic over the **STARK252** and **NIST P-256** prime fields.

## Features
This library provides core elliptic curve primitives and optimized multiplication algorithms:

*   **Group Operations**: Affine coordinate implementation of point addition and point doubling.
*   **Fixed-Base Multiplication**: Efficient scalar multiplication for the curve generator using precomputed power-of-two tables ($2^i \cdot G$).
*   **Variable-Base Multiplication**: Scalar multiplication for arbitrary points using a 4-bit (radix-16) windowed method to reduce the number of point additions.
*   **Trait-Based Architecture**: Extensible design using the `CurveConfig` trait, allowing for the addition of new curves by defining field parameters and generator coordinates.

## Supported Curves
| Curve | Field | Use Case |
| --- | --- | --- |
| **STARK252** | $2^{251} + 17 \cdot 2^{192} + 1$ | Starknet, ZK-STARKs, FRI |
| **NIST P-256** | $2^{256} - 2^{224} + 2^{192} + 2^{96} - 1$ | TLS/SSL, WebAuthn |

## Mathematical Implementation
The library operates on short Weierstrass curves defined by $y^2 = x^3 + ax + b$.

### Scalar Multiplication Algorithms
1.  **Fixed-Base (Generator)**:
    Since the generator for a group on a fixed elliptic curve is constant, the library utilizes a precomputed table of $[2^i]G$ for $i \in [0, 255]$. Scalar multiplication is reduced to a maximum of 256 additions, eliminating doubling operations during execution.
2.  **Variable-Base (Arbitrary Point)**:
    Uses a 4-bit window (nibble-based) approach. A small local table of $16$ points ($[0]P \dots [15]P$) is generated on-the-fly. The scalar is processed from most-significant to least-significant nibble, performing 4 doublings and 1 addition per nibble.

## Testing
The test suite validates the implementation against the following criteria:
*   **Curve Consistency**: Verifying that the generator and calculated points satisfy the curve equation.
*   **Group Law**: Testing identity properties, point inversion, and associativity.
*   **Scalar Order**: Confirming that $[n]P = \mathcal{O}$ where $n$ is the prime order of the subgroup.

Run tests with:
```bash
cargo test

```

## Benchmarking

Performance is measured using Criterion. The suite compares the efficiency of the different multiplication strategies across the supported fields.

Run benchmarks with:

```bash
cargo bench

```

## Dependencies

* `ff`: For finite field arithmetic traits.
* `num-bigint`: For arbitrary-precision scalar arithmetic.
* `criterion`: For performance analysis.
