use std::rand::random;
use block::{
    Block,
    EmptyBlock,
    ColorBlock,
};

pub struct Tetromino {
    pub blocks: Vec<Vec<Vec<Block>>>,
}

pub struct TetrominoManager {
    tetrominoes: Vec<Tetromino>,
    curr_tetromino_idx: (uint, uint),
    next_tetromino_idx: (uint, uint),
}

impl TetrominoManager {
    pub fn new() -> TetrominoManager {
        let mut manager = TetrominoManager {
            tetrominoes: vec![],
            curr_tetromino_idx: (0,0),
            next_tetromino_idx: (0,0),
        };

        let empty = EmptyBlock;

        // I
        let mut block = ColorBlock(1.0, 1.0, 0.0);
        manager.tetrominoes.push(Tetromino {
            blocks: vec![vec![vec![block, block, block, block]],
                         vec![vec![block],
                              vec![block],
                              vec![block],
                              vec![block]]],
        });

        // O
        block = ColorBlock(1.0, 0.0, 0.0);
        manager.tetrominoes.push(Tetromino {
            blocks: vec![vec![vec![block, block],
                              vec![block, block]]],
        });

        // T
        block = ColorBlock(1.0, 0.0, 1.0);
        manager.tetrominoes.push(Tetromino {
            blocks: vec![vec![vec![empty, block , empty],
                              vec![block, block, block]],
                         vec![vec![block, empty],
                              vec![block, block],
                              vec![block, empty]],
                         vec![vec![block, block, block],
                              vec![empty, block, empty]],
                         vec![vec![empty, block],
                              vec![block, block],
                              vec![empty, block]]],
        });

        // J
        block = ColorBlock(0.0, 1.0, 0.0);
        manager.tetrominoes.push(Tetromino {
            blocks: vec![vec![vec![block, empty, empty],
                              vec![block, block, block]],
                         vec![vec![block, block],
                              vec![block, empty],
                              vec![block, empty]],
                         vec![vec![block, block, block],
                              vec![empty, empty, block]],
                         vec![vec![empty, block],
                              vec![empty, block],
                              vec![block, block]]],
        });

        // L
        block = ColorBlock(0.0, 1.0, 1.0);
        manager.tetrominoes.push(Tetromino {
            blocks: vec![vec![vec![empty, empty, block],
                              vec![block, block, block]],
                         vec![vec![block, empty],
                              vec![block, empty],
                              vec![block, block]],
                         vec![vec![block, block, block],
                              vec![empty, empty, block]],
                         vec![vec![block, block],
                              vec![empty, block],
                              vec![empty, block]]],
        });

        // S
        block = ColorBlock(0.0, 0.0, 1.0);
        manager.tetrominoes.push(Tetromino {
            blocks: vec![vec![vec![empty, block, block],
                              vec![block, block, empty]],
                         vec![vec![block, empty],
                              vec![block, block],
                              vec![empty, block]]],
        });

        // Z
        block = ColorBlock(1.0, 1.0, 1.0);
        manager.tetrominoes.push(Tetromino {
            blocks: vec![vec![vec![block, block, empty],
                              vec![empty, block, block]],
                         vec![vec![empty, block],
                              vec![block, block],
                              vec![block, empty]]],
        });

        manager
    }

    pub fn generate_next(&mut self) {
        self.curr_tetromino_idx = self.next_tetromino_idx;
        self.next_tetromino_idx = (random::<uint>() % self.tetrominoes.len(), 0);
    }

    pub fn get_curr_tetromino(&self) -> (&Tetromino, uint) {
        match self.curr_tetromino_idx {
            (i, j) => (&self.tetrominoes[i], j)
        }
    }

    pub fn rotate_tetromino(&mut self) {
        self.curr_tetromino_idx = match self.curr_tetromino_idx {
            (i, j) => {
                if self.tetrominoes[i].blocks.len() <= (j + 1) {
                    (i, 0)
                } else {
                    (i, j + 1)
                }
            }
        };
    }

    pub fn get_width(&self) -> uint {
        match self.get_curr_tetromino() {
            (tetromino, v) => {
                tetromino.blocks[v][0].len()
            }
        }
    }

    pub fn get_height(&self) -> uint {
        match self.get_curr_tetromino() {
            (tetromino, v) => {
                tetromino.blocks[v].len()
            }
        }
    }
}
