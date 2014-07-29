use std::rand::random;

pub struct Tetromino {
    pub blocks: Vec<Vec<Vec<bool>>>,
    pub color: (f32, f32, f32, f32)
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

        // I
        manager.tetrominoes.push(Tetromino {
            blocks: vec![vec![vec![true, true, true, true]],
                         vec![vec![true],
                              vec![true],
                              vec![true],
                              vec![true]]],
            color: (1.0, 1.0, 0.0, 1.0)
        });

        // O
        manager.tetrominoes.push(Tetromino {
            blocks: vec![vec![vec![true, true],
                              vec![true, true]]],
            color: (1.0, 0.0, 0.0, 1.0)
        });

        // T
        manager.tetrominoes.push(Tetromino {
            blocks: vec![vec![vec![false, true , false],
                              vec![true , true , true ]],
                         vec![vec![true , false],
                              vec![true , true ],
                              vec![true , false]],
                         vec![vec![true , true , true ],
                              vec![false, true , false]],
                         vec![vec![false, true],
                              vec![true , true],
                              vec![false, true]]],
            color: (1.0, 0.0, 1.0, 1.0)
        });

        // J
        manager.tetrominoes.push(Tetromino {
            blocks: vec![vec![vec![true , false, false],
                              vec![true , true , true ]],
                         vec![vec![true , true ],
                              vec![true , false],
                              vec![true , false]],
                         vec![vec![true , true , true],
                              vec![false, false, true]],
                         vec![vec![false, true],
                              vec![false, true],
                              vec![true , true]]],
            color: (0.0, 1.0, 0.0, 1.0)
        });

        // L
        manager.tetrominoes.push(Tetromino {
            blocks: vec![vec![vec![false, false, true],
                              vec![true , true , true]],
                         vec![vec![true , false],
                              vec![true , false],
                              vec![true , true ]],
                         vec![vec![true , true , true],
                              vec![false, false, true]],
                         vec![vec![true , true],
                              vec![false, true],
                              vec![false, true]]],
            color: (0.0, 1.0, 1.0, 1.0)
        });

        // S
        manager.tetrominoes.push(Tetromino {
            blocks: vec![vec![vec![false, true , true ],
                              vec![true , true , false]],
                         vec![vec![true , false],
                              vec![true , true ],
                              vec![false, true ]]],
            color: (0.0, 0.0, 1.0, 1.0)
        });

        // Z
        manager.tetrominoes.push(Tetromino {
            blocks: vec![vec![vec![true , true , false],
                              vec![false, true , true ]],
                         vec![vec![false, true ],
                              vec![true , true ],
                              vec![true , false]]],
            color: (1.0, 1.0, 1.0, 1.0)
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
