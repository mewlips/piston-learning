#[deriving(Clone)]
pub enum Block {
    EmptyBlock,
    ColorBlock(f32, f32, f32),
}
