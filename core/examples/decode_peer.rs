use libp2p::PeerId;

fn main() {
    let hex_str = "0024080112206ac4c2ee4703c157754cf86370d9d8dcbbb5e4bf31adf596fbd72eed2d4f0e69";
    let bytes = hex::decode(hex_str).expect("Invalid hex");
    
    println!("Hex: {}", hex_str);
    println!("Bytes: {} bytes", bytes.len());
    
    match PeerId::from_bytes(&bytes) {
        Ok(peer_id) => {
            println!("Base58: {}", peer_id.to_base58());
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
