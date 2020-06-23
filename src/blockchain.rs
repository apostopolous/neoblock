use super::*;

pub struct Blockchain {
  pub blocks: Vec<Block>,
}

impl Blockchain {
  pub fn verify(&self) -> bool {
    for (i, block) in self.blocks.iter().enumerate() {
      if block.index != i as u32 {
        println!("Index of block is not equal to block's position within blockchain.");
        return false;
      } else if !block::check_difficulty(&block.hash(), block.difficulty) {
        println!("Block difficulty is not greater than hash difficulty.");
        return false;
      } else if i != 0 {
        // Not first block
        let previous_block = &self.blocks[i - 1];
        if block.timestamp <= previous_block.timestamp {
          println!("Time of previous block is not less than time of current block.");
          return false;
        } else if block.previous_hash != previous_block.hash {
          println!(
            "Hash disparity error, current block's hash is not equal to previous block's hash."
          );
          return false;
        }
      } else {
        // First block
        if block.previous_hash != vec![0; 32] {
          println!("Invalid previous hash for first block.");
          return false;
        }
      }
    }

    true
  }
}
