use block::{
    Block,
    EmptyBlock,
    ColorBlock,
};

pub struct Space {
    board: Vec<Vec<Block>>
}

impl Space {
    pub fn new(width: u32, height: u32) -> Space {
        let mut board: Vec<Vec<Block>> = Vec::new();

        for i in range(0u32, height) {
            let mut line: Vec<Block> = Vec::new();
            line.grow(width as uint, &EmptyBlock);
            board.push(line);
        }

        // TEST!!
        *board.get_mut(18).get_mut(3) = ColorBlock(1.0, 0.0, 0.0);
        //board.get_mut(18).get_mut(4).set(1.0, 1.0, 0.0);

        Space {
            board: board
        }
    }

    pub fn get_block_ref(&self, x: uint, y: uint) -> &Block {
        &self.board[y][x]
    }
}
