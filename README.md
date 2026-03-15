# GateLink Protocol: Smart Contracts 🎟️⚡

This repository contains the core Soroban (Stellar) smart contracts that power the **GateLink decentralized micro-ticketing protocol**.

Built under **gates-labs**, GateLink enables instant **USDC settlement** for event organizers and transit operators while issuing **non-transferable, lightweight digital tickets** to users.

---

# 🎯 Repository Focus

This repo handles all **on-chain logic**, including:

- Minting and managing digital ticket assets (Soulbound / Non-transferable tokens)
- Atomic settlement of USDC payments directly to organizers
- On-chain access control and event state management

---

# 🛠 Prerequisites

This project is built with **Rust** for the **Soroban smart contract platform**.

Ensure your **development environment** has the necessary toolchain.

## Install Rust Toolchain

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-unknown-unknown
```

## Install Soroban CLI

```bash
cargo install --locked soroban-cli
```

## Install Docker (for local Stellar network)

```bash
sudo apt-get update
sudo apt-get install docker.io
```

---

# 🚀 Quickstart

## Clone the repository

```bash
git clone https://github.com/gates-labs/gatelink-contracts.git
cd gatelink-contracts
```

## Build the contracts to WebAssembly

```bash
soroban contract build
```

## Run the test suite

```bash
cargo test
```

---

# 🤝 Contributing

Contributions are welcome!

Please check the **open issues** or open a new one to discuss proposed changes before submitting a pull request.

Ensure that:

- Your code passes all existing tests
- New features include corresponding tests