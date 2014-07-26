use std::rand::random;
use std::cmp::max;

pub type Block = Vec<bool>;
pub type BlockSet =  Vec<Block>;

pub struct BlockManager {
    blocks: Vec<BlockSet>,
    curr_block_idx: (uint, uint),
    next_block_idx: (uint, uint),
}

impl BlockManager {
    pub fn new() -> BlockManager {
        let mut block_manager = BlockManager {
            blocks:
                vec![
                    vec![vec![false, false, false, false,
                              false, false, false, false,
                              true , true , false, false,
                              true , true , false, false]],
                    vec![vec![false, false, false, false,
                              false, false, false, false,
                              true , false, false, false,
                              true , true , true , false],
                         vec![false, false, false, false,
                              true , true , false, false,
                              true , false, false, false,
                              true , false, false, false],
                         vec![false, false, false, false,
                              false, false, false, false,
                              true , true , true , false,
                              false, false, true , false],
                         vec![false, false, false, false,
                              false, true , false, false,
                              false, true , false, false,
                              true , true , false, false]],
                    vec![vec![false, false, false, false,
                              false, false, false, false,
                              false, false, true , false,
                              true , true , true , false],
                         vec![false, false, false, false,
                              true , false, false, false,
                              true , false, false, false,
                              true , true , false, false],
                         vec![false, false, false, false,
                              false, false, false, false,
                              true , true , true , false,
                              false, false, true , false],
                         vec![false, false, false, false,
                              true , true , false, false,
                              false, true , false, false,
                              false, true , false, false]],
                    vec![vec![false, false, false, false,
                              false, false, false, false,
                              false, true , false, false,
                              true , true , true , false],
                         vec![false, false, false, false,
                              true , false, false, false,
                              true , true , false, false,
                              true , false, false, false],
                         vec![false, false, false, false,
                              false, false, false, false,
                              true , true , true , false,
                              false, true , false, false],
                         vec![false, false, false, false,
                              false, true , false, false,
                              true , true , false, false,
                              false, true , false, false]],
                    vec![vec![false, false, false, false,
                              false, false, false, false,
                              false, false, false, false,
                              true , true , true , true ],
                         vec![true , false, false, false,
                              true , false, false, false,
                              true , false, false, false,
                              true , false, false, false]],

                ],
            curr_block_idx: (0,0),
            next_block_idx: (0,0),
        };

        for ref block_set in block_manager.blocks.iter() {
            for ref block in block_set.iter() {
                assert!(block.len() == 16);
            }
        }

        block_manager.get_next_block(); // next_block is randomized.
        block_manager.get_next_block(); // curr_block is randomized.

        block_manager
    }

    pub fn get_next_block(&mut self) {
        self.curr_block_idx = self.next_block_idx;
        self.next_block_idx = (random::<uint>() % self.blocks.len(), 0);
    }

    pub fn get_curr_block(&self) -> &Block {
        match self.curr_block_idx {
            (n, m) => self.blocks.get(n).get(m)
        }
    }

    pub fn rotate_block(&mut self) {
        self.curr_block_idx = match self.curr_block_idx {
            (n, m) => {
                if self.blocks.get(n).len() <= (m + 1) {
                    (n, 0)
                } else {
                    (n, m + 1)
                }
            }
        };
    }

    pub fn get_width(&self) -> uint {
        let curr_block = self.get_curr_block();
        let mut width = 0;

        for y in range(0u, 4) {
            for x in range(0u, 4) {
                if *curr_block.get(y*4 + x) {
                    width = max(width, x+1);
                }
            }
        }

        width
    }

    pub fn get_height(&self) -> uint {
        let curr_block = self.get_curr_block();

        if curr_block.slice_to(12) == [false, false, false, false,
                                      false, false, false, false,
                                      false, false, false, false] {
            1
        } else if curr_block.slice_to(8) == [false, false, false, false,
                                            false, false, false, false] {
            2
        } else if curr_block.slice_to(4) == [false, false, false, false] {
            3
        } else {
            4
        }
    }
}
