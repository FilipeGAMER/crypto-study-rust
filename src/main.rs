use sha256::digest;

use std::time::SystemTime;

#[derive(Debug)]
struct Data {
    sender: String,
    recipient: String,
    quantity: u64
}

#[derive(Debug)]
struct Block {
    index: u128,
    timestamp: SystemTime,
    data: Data,
    previous_hash: String,
    hash: String
}

impl Block {
    fn new(index: u128, timestamp: SystemTime, data: Data, previous_hash: String) -> Block {
        let mut bl = Block {
            index: index,
            timestamp: timestamp,
            data: data,
            previous_hash: previous_hash,
            hash: String::from("")
        };

        bl.hash = bl.generate_hash();
        
        return bl;
    }

    fn generate_hash(&self) -> String {
        let to_hash = format!("{}{}{}{}",self.index,stringify!(self.timestamp),self.previous_hash,stringify!(self.data));

        let result = digest(to_hash);
        return result
    }
}

struct Blockchain {
    blockchain: Vec<Block>
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let mut vec: Vec<Block> = Vec::new();
        vec.push(Blockchain::create_genesis_block());
        Blockchain {
            blockchain: vec
        }
    }

    fn create_genesis_block() -> Block {
        return Block::new(
            0, 
            SystemTime::now(), 
            Data {
                sender: "FromFi".to_string(),
                recipient: "TestFi".to_string(),
                quantity: 14
            },
            "0".to_string()
        );
    }

    fn get_latest_block_hash(&self) -> &String {
        return &self.blockchain[self.blockchain.len() -1].hash;
    }

    pub fn add_new_block(&mut self, index: u128, timestamp: SystemTime, data: Data) {

        let previous_hash = self.get_latest_block_hash();
        let new_block = Block::new(
            index, 
            timestamp, 
            data,
            previous_hash.to_owned()
        );

        self.blockchain.push(new_block);
    }
}


fn main() {
    let mut bazzuca_coin = Blockchain::new();

    bazzuca_coin.add_new_block(
        1, 
        SystemTime::now(), 
        Data {
            sender: "FromFi".to_string(),
            recipient: "TestFi".to_string(),
            quantity: 14
    });
    
    bazzuca_coin.add_new_block(
        1, 
        SystemTime::now(), 
        Data {
            sender: "FromFi 1".to_string(),
            recipient: "TestFi 1".to_string(),
            quantity: 25
    });
    
    bazzuca_coin.add_new_block(
        1, 
        SystemTime::now(), 
        Data {
            sender: "FromFi 2".to_string(),
            recipient: "TestFi 2".to_string(),
            quantity: 18
    });
    
    println!("{:#?}", bazzuca_coin.blockchain);
}
