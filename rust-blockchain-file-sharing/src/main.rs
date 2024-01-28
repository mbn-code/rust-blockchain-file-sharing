
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize)]
struct BlockConfig {
  network: String,
}

fn read_blockchain_conf() -> Result<BlockConfig, Box<dyn std::error::Error>> {
  let mut file = File::open("blockchain.conf")?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;
  
  let config: BlockConfig = serde_yaml::from_str(&contents)?;
  
  Ok(config)
}

fn main() {
    match read_blockchain_conf() {
        Ok(config) => println!("{:?}", config),
        Err(e) => println!("Error: {:?}", e),
    }
}

