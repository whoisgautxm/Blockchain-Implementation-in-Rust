use chrono::Utc;
use log::info;
use log::warn;
use sha2::Digest;
use sha2::Sha256;
use log::error;

const DIFFICULTY_PREFIX: &'static str = "0000";
pub struct App {
    pub blocks: Vec<Block>,
}

pub struct Block {
    pub id: u64,
    pub data: String,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: i64,
    pub nonce: u64,
}

fn calculate_hash(id: u64, timestamp: i64, previous_hash: &String, data: &String, nonce: u64) -> Vec<u8> {
    let data = serde_json::json!({
        "id": id,
        "previous_hash": previous_hash,
        "data": data,
        "timestamp": timestamp,
        "nonce": nonce
    });
    let mut hasher = Sha256::new();
    hasher.update(data.to_string().as_bytes());
    hasher.finalize().as_slice().to_owned()
}

fn mine_block(id: u64, timestamp: i64, previous_hash: &String, data: &String) -> (u64, String) {
    info!("mining a block");
    let mut nonce: u64 = 0;
    loop {
        if nonce % 10000 == 0 {
            info! ("none = {nonce}");
        }
        let hash = calculate_hash(id, timestamp, previous_hash, data, nonce);
        let binary_hash = hash_to_binary_string(&hash);
        if binary_hash.starts_with(DIFFICULTY_PREFIX) {
            info!(
                "block mined!, nonce : {}, hash: {}, binary hash: {}", nonce, hex::encode(&hash), binary_hash
            );
            return (nonce, hex::encode(binary_hash));
        }
        nonce +=1;
    }
}

fn hash_to_binary_string(hash: &[u8]) -> String {
    let mut res = String::from("");
    for n in hash {
        res.push_str(&format!("{:b}", n));
    }
    res
}

impl App {
    fn new() -> Self {
        Self { blocks: vec![] }
    }

    fn genesis(&mut self) {
        let genesis = Block {
            id: 0,
            data: String::from("genesis"),
            hash: "0000f0e671ceac529fee3f68db0ae3937a85e23b3c98d51a1a7796c3c042f17a".to_string(),
            previous_hash: "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            timestamp: Utc::now().timestamp(),
            nonce: 69,
        };
        self.blocks.push(genesis);
    }

    fn add_new_block(&mut self, block: Block) {
        let last_block = self.blocks.last().expect("there is atleast one block");
        if self.is_block_valid(&block, last_block) {
            self.blocks.push(block);
        } else {
            error!("invalid block");
        }
    }

    fn is_block_valid(&self, block: &Block, previous_block: &Block) -> bool {
        if block.previous_hash != previous_block.hash {
            warn!("invalid block, block's hash doesnt match previous block's hash");
            false
        } else if !(hash_to_binary_string(&hex::decode(&block.hash).expect("decode block hash"))
            .starts_with(DIFFICULTY_PREFIX))
        {
            false
        } else if (block.id != previous_block.id + 1) {
            false
        } else if hex::encode(calculate_hash(
            block.id,
            block.timestamp,
            &block.previous_hash,
            &block.data,
            block.nonce,
        )) != block.hash
        {
            false
        } else {
            true
        }
    }

    fn is_chain_valid(&self, chain: &[Block]) -> bool {
        for i in 0..chain.len() {
            if i == 1 {
                continue;
            } else {
                let second = chain.get(i - 1).expect("has to exist");
                let first = chain.get(i).expect("has to exist");
                if !self.is_block_valid(second, first) {
                    false;
                }
            }
        }
        true
    }

    fn choose_chain(&self, local: Vec<Block>, remote: Vec<Block>) -> Vec<Block>{
        let is_local_valid = self.is_chain_valid(&local);
        let is_remote_valid = self.is_chain_valid(&remote);

        if is_local_valid && is_remote_valid {
            if local.len() > remote.len() {
                local
            } else {
                remote
            }
        } else if !is_local_valid && is_remote_valid {
            remote
        } else if !is_remote_valid && is_local_valid {
            local
        } else {
            panic!("both local and remote chain are invalid");
        }
    }

    
}

impl Block {
    pub fn new(id: u64, previous_hash: &String, data: &String) -> Self {
        let timestamp: i64 = Utc::now().timestamp(); 
        let (nonce, hash) = mine_block(id, timestamp, previous_hash, data) ;
        Self {
            id,
            previous_hash: previous_hash.to_string(),
            data: data.to_string(),
            timestamp,
            nonce,
            hash,
        }
    }
}
fn main() {}