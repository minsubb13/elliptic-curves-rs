# Elliptic Curve Cryptography

A comprehensive implementation of elliptic curve cryptography primitives, protocols, and cryptanalysis algorithms in Rust.

## Purpose
This repository is for:

- **Practice Implementing ECC Theory**

    Translating theoretical knowledge of elliptic curve cryptography into working code, base on this post [Elliptic Curve Cryptography: a gentle introduction](https://andrea.corbellini.name/2015/05/17/elliptic-curve-cryptography-a-gentle-introduction/).

- **Rust Language Study**

    **Rust is becoming the preferred language in the crpytography domain** due to its unique combination of **memory safety**, **performance**, and **type safety**.
    
    This project serves as a comprehensive way to learn Rust by implementing a substantial cryptographic system.

- **Unified Library Crate**

    **Basic operations for big integers are extremely difficult to implement correctly.** Proven libraries provide this functionality flawlessly. By actively utilizing libraries that offer these capabilities, **I delegated the finite field arithmetic operations required for ECC to established libraries**, which allowed me to practice library integration skills.

- **Trunk-Based Development Practice**

    This project was implemented using **trunk-based development** methodology. I maintained only a single main branch and actively utilized GitHub's collaborative features:
    
    - **Issues and Pull Requests** - Systematic tracking of features and bug fixes
    - **Code Reviews** - Ensuring code quality through peer review processes (But, I can't review my code myself due to GitHub's policy)
    - **Build Testing** - When sending PR or merged, It must pass the build test
    - **Kanban Board** - Visual project management and workflow tracking

This is a reimplementation and extension of my previous [C++ version](https://github.com/minsubb13/elliptic-curve-cryptography), built with Rust practices and enhanced functionality.

## Features

### Implemented

- **Finite Field Arithmetic**: Prime field operations with big integer support
- **Elliptic Curve Operations**: Point addition, doubling, scalar multiplication
- **Real Curve Parameters**: Full implementation of Secp256k1 curve
- **ECDH Protocol**: Elliptic Curve Diffie-Hellman key exchange
- **ECDSA Protocol**: Digital signature generation and verification
- **Discrete Logarithm Attacks**:
    - Brute force algorithm
    - Pollard's rho algorithm
    - Baby-step Giant-step (placeholder)

### Dependencies

- `ark-ff`: Finite field arithmetic and big integer operations
- `sha2`: SHA-256 hashing for ECDSA
- `rand`: Cryptographically secure random number generation

## Project Structure

```
src/
├── core/                   # Core mathematical abstractions
│   ├── field.rs            # Field and PrimeField traits
│   ├── curve.rs            # Elliptic curve trait definition
│   └── point.rs            # Point arithmetic and operations
├── curves/                 # Specific curve implementations
│   └── secp256k1/          # secp256k1 curve implementation
│                           # other curves ...
├── protocols/              # Cryptographic protocols
│   ├── ecdh.rs             # Elliptic Curve Diffie-Hellman
│   └── ecdsa.rs            # Elliptic Curve Digital Signature Algorithm
└── breaking_dlp/           # Discrete logarithm cryptanalysis
    ├── brute_force.rs      # O(n) brute force attack
    ├── pollards_rho.rs     # O(√n) Pollard's rho algorithm
    └── baby_step_giant_step.rs  # O(√n) BSGS algorithm (TODO)
```