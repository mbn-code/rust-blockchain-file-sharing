# Rust Blockchain File Sharing

[![Rust](https://img.shields.io/badge/language-Rust-orange)](https://www.rust-lang.org/)
[![Solidity](https://img.shields.io/badge/language-Solidity-brightgreen)](https://soliditylang.org/)
[![Makefile](https://img.shields.io/badge/language-Makefile-blue)](https://www.gnu.org/software/make/)

**Securely share files on the blockchain with this Rust-based application.**

## Overview
This project is a decentralized file-sharing application that ensures the confidentiality and integrity of shared files. Developed for cross-platform compatibility on Windows and macOS.

## Features
- **User Authentication:** Implements user authentication for secure access to file-sharing functionalities.
- **Ethereum Integration:** Utilizes Ethereum smart contracts to manage file storage and sharing securely.
- **Cross-Platform Compatibility:** Developed to work seamlessly on both Windows and macOS environments.

## Languages
1. **Rust: 72.6%**
2. **Solidity: 14.4%**
3. **Makefile: 13.0%**

## Usage
1. **Local Ethereum Node:**
   - Ensure you have a local Ethereum node running.

2. **Modify Contract Address:**
   - Update `CONTRACT_ADDRESS` in `file_sharing.rs` with the actual smart contract address.

3. **Run the Application:**
   - Execute the Rust program to securely share files on the blockchain.

## Project Structure
- **main.rs:** Entry point for the application, initiating file sharing.
- **user_authentication.rs:** Manages user authentication using a simulated user database.
- **file_sharing.rs:** Interacts with Ethereum smart contracts to handle file sharing securely.
- **file.rs:** Defines the structure of a file, including its name and content.

## Security
> [!CAUTION]
This is a simplified example and should be adapted for production use, incorporating additional security measures and thorough testing.

> [!IMPORTANT]
Ensure responsible usage and adhere to legal and ethical considerations when sharing files.
