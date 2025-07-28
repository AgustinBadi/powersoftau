# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Powers of Tau Multi-Party Computation (MPC) ceremony implementation in Rust for generating zk-SNARK parameters. It constructs partial zk-SNARK parameters for circuits up to depth 2^21 using the BLS12-381 elliptic curve construction. The ceremony allows multiple participants to contribute randomness to generate trusted setup parameters.

## Build and Development Commands

### Basic Setup
```bash
rustup update
cargo build
cargo build --release
```

### Running Ceremony Components
- `cargo run --release --bin new_constrained` - Generate initial empty challenge
- `cargo run --release --bin compute_constrained` - Participant computation (generates response from challenge)  
- `cargo run --release --bin verify_transform_constrained` - Verify transformation and generate new challenge
- `cargo run --release --bin beacon_constrained` - Apply randomness beacon at ceremony end

### Testing
- `./test.sh` - Full ceremony simulation script that runs through complete ceremony workflow

### Nix Development
- `nix develop` - Enter development shell with Rust toolchain
- `nix build` - Build the project with Nix

## Architecture

### Core Components

**Ceremony Binaries** (`src/bin/`):
- `new_constrained.rs` - Initializes ceremony with empty accumulator
- `compute_constrained.rs` - Participant contribution computation  
- `verify_transform_constrained.rs` - Verifies contributions and updates accumulator
- `beacon_constrained.rs` - Applies final randomness beacon

**Core Libraries**:
- `batched_accumulator.rs` - Main accumulator data structure for ceremony state
- `parameters.rs` - Configuration and parameter definitions for ceremony
- `keypair.rs` - Key generation and management utilities
- `small_bls12_381/` - BLS12-381 curve-specific implementations
- `utils.rs` - Utility functions including hashing

### Ceremony Workflow

The ceremony follows this sequence:
1. Coordinator generates initial challenge using `new_constrained`
2. Each participant runs `compute_constrained` to contribute randomness
3. Coordinator verifies each contribution with `verify_transform_constrained`
4. After all participants, coordinator applies beacon with `beacon_constrained`
5. Final verification produces the trusted setup parameters

### File Artifacts
- `challenge` - Current ceremony state for participants
- `response` - Participant's contribution output
- `new_challenge` - Verified updated ceremony state

## Key Dependencies
- `bellman` - zk-SNARK library providing curve arithmetic and proving system
- `blake2` - Cryptographic hashing for participant commitments
- `memmap` - Memory-mapped file I/O for handling large ceremony files
- `crossbeam` - Parallel processing utilities