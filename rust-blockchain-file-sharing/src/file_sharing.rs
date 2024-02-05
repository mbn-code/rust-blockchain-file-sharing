// file_sharing.rs

use crate::user_authentication::authenticate_user;
use crate::file::File;
use std::env;
use std::io::{self, Write};
use web3::transports::Http;
use web3::types::{Block, Transaction};
use web3::Web3;

// Ethereum smart contract ABI (generated from compilation)
const FILE_SHARING_ABI: &[u8] = include_bytes!("rust-blockchain-file-sharing/src/build/contracts/FileSharing.json");

// Ethereum smart contract address
const CONTRACT_ADDRESS: &str = "0xYourContractAddress";

// Initialize Web3 connection
fn init_web3() -> Web3<Http> {
    let http = Http::new("http://localhost:8545").expect("Failed to connect to Ethereum node");
    Web3::new(http)
}

// Share file by interacting with the Ethereum smart contract
pub fn share_file() {
    println!("Enter your Ethereum address:");
    let mut address_input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut address_input).unwrap();
    let sender_address = address_input.trim();

    println!("Enter the file name:");
    let mut name_input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name_input).unwrap();
    let file_name = name_input.trim();

    println!("Enter the file content:");
    let mut content_input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut content_input).unwrap();
    let file_content = content_input.trim();

    if authenticate_user(sender_address, "password") {
        let web3 = init_web3();

        // Encode function data for Ethereum smart contract call
        let function_data = web3::contract::Contract::encode_input(
            FILE_SHARING_ABI,
            "uploadFile",
            (file_name, file_content),
        )
        .expect("Failed to encode function data");

        // Create a signed Ethereum transaction
        let transaction = TransactionRequest {
            gas: Some(100000.into()),
            gas_price: Some(1000000000.into()), // Adjust gas price accordingly
            input: Some(function_data.into()),
            ..Default::default()
        };
        

        // Send the transaction to the Ethereum network
        let transaction_hash = web3
            .eth()
            .send_transaction(transaction)
            .expect("Failed to send Ethereum transaction");

        // Wait for the transaction to be mined
        let _ = web3
            .eth()
            .await_transaction(
                transaction_hash,
                Some(web3::transports::Event::Transaction),
                Some(Block::Pending),
            )
            .expect("Failed to await Ethereum transaction");

        println!("File shared successfully!");
    } else {
        println!("Authentication failed. Unable to share the file.");
    }
}
