extern crate crypto;
extern crate time;

use crypto::digest::Digest;
use crypto::sha2::Sha512;

use std::io;

fn main() {
    let mut blockchain = Vec::new();
    blockchain.push(get_genesis_block());
    
    let mut data = String::new();
    println!("Enter the data: ");
    io::stdin().read_line(&mut data)
        .expect("Failed to read line");
    {
        let block = generate_next_block(data, &blockchain[0]);
        blockchain.push(block);
    }
}

pub struct Block{
    index: i64,
    timestamp: String,
    data: String,
    hash: String,
    previous_hash: String,
}

/*fn calculate_hash_for_block(block: &Block) -> String {
    calculate_hash(block.index, &block.previous_hash, block.timestamp, &block.data)
}*/

fn calculate_hash(index: i64, p_hash: &String, timestamp: String, data: &String) -> String {
    let mut hasher = Sha512::new();
    let str_final = index.to_string() + &p_hash + &timestamp + &data;
    hasher.input_str(&str_final);
    return hasher.result_str();
}

fn generate_next_block(data: String, p_block: &Block) -> Block {
    let index = p_block.index + 1;
    let timestamp = time::now().to_timespec();
        
    let hash = calculate_hash(index, &p_block.hash, timestamp.sec.to_string(), &data);
        
    return Block{
        index: index, 
        previous_hash: p_block.hash.clone(),
        timestamp: timestamp.sec.to_string(),
        data: data,
        hash: hash

    };
}

//Genesis block is the first block. Always
fn get_genesis_block() -> Block {
    Block{ 
    index: 0,
    previous_hash: String::from("0"),
    timestamp: String::from("1465154705"),
    data: String::from("my genesis block!!"),
    hash: String::from("816534932c2b7154836da6afc367695e6337db8a921823784c14378abed4f7d7")
    }
}

fn is_valid_new_block(n_block: Block, p_block: Block) -> bool {
    let hash = calculate_hash(n_block.index, &n_block.previous_hash, n_block.timestamp, &n_block.data);
    if p_block.index + 1 != n_block.index {
        println!("Invalid index");
        return false;
    }
    else if p_block.hash != n_block.previous_hash{
        println!("Invalid previous hash");
        return false;
    }
    else if hash != n_block.hash{
        println!("Invalid hash");
        return false;
    }
    return true;
}

//Conflict with two chains hanlding should go here
//fn replace_chain(newBlocks: Vec<Block>) 
