use glfw_game_window::GameWindowGLFW;
use opengl_graphics::Gl;
use piston::{
    Game,
    RenderArgs,
    UpdateArgs,
    MouseMoveArgs,
    MousePressArgs,
};
use piston::mouse::{
    Left,
    Right,
};
use graphics::{
    Context,
    AddRectangle,
    AddColor,
    Draw,
    RelativeTransform2d,
};
use block::{
    EmptyBlock,
    ColorBlock,
};
use space::Space;
use tetromino::{
    TetrominoManager,
};

pub static BLOCK_SIZE:   u32 = 40;
pub static BLOCK_WIDTH:  u32 = 10;
pub static BLOCK_HEIGHT: u32 = 20;

pub struct App {
    gl: Gl,        // OpenGL drawing backend.
    tetromino_manager: TetrominoManager,
    space: Space,
    x: f64,
    y: f64,
}

impl App {
    pub fn new() -> App {
        App {
            gl: Gl::new(),
            tetromino_manager: TetrominoManager::new(),
            space: Space::new(BLOCK_WIDTH, BLOCK_HEIGHT),
            x: 0.0,
            y: 0.0,
        }
    }

    fn draw_board(&mut self, context: &Context) {
        for x in range(0u, BLOCK_WIDTH as uint) {
            for y in range(0u, BLOCK_HEIGHT as uint) {
                let block = self.space.get_block_ref(x, y);
                match block {
                    &EmptyBlock => {}
                    &ColorBlock(r, g, b) => {
                        context
                            .rect((x as u32 * BLOCK_SIZE) as f64,
                                  (y as u32 * BLOCK_SIZE) as f64, 
                                  BLOCK_SIZE as f64,
                                  BLOCK_SIZE as f64)
                            .rgba(r, g, b, 1.0)
                            .draw(&mut self.gl);
                    }
                }
            }
        }
    }

    fn draw_tetromino(&mut self, context: &Context) {
        let width = self.tetromino_manager.get_width();
        let height = self.tetromino_manager.get_height();
        let (tetromino, v) = self.tetromino_manager.get_curr_tetromino();
        let blocks = &tetromino.blocks[v];

        for y in range(0u, height) {
            for x in range(0u, width) {
                match &blocks[y][x] {
                    &EmptyBlock => {}
                    &ColorBlock(r, g, b) => {
                        context
                            .rect((x as u32 * BLOCK_SIZE) as f64,
                                  (y as u32 * BLOCK_SIZE) as f64,
                                  BLOCK_SIZE as f64,
                                  BLOCK_SIZE as f64)
                            .rgba(r, g, b, 1.0)
                            .trans(self.x, self.y)
                            .draw(&mut self.gl);
                    }
                }
            }
        }
    }
}

impl Game<GameWindowGLFW> for App {
    fn render(&mut self, _window: &mut GameWindowGLFW, args: &RenderArgs) {
        let context = &Context::abs(args.width as f64, args.height as f64);
        context.rgba(0.0, 0.0, 0.0, 1.0).draw(&mut self.gl);
        self.draw_board(context);
        self.draw_tetromino(context);
    }

    fn update(&mut self, _window: &mut GameWindowGLFW, _args: &UpdateArgs) {
        self.y += 1.0;
    }

    fn mouse_move(&mut self, _window: &mut GameWindowGLFW, args: &MouseMoveArgs) {
        self.x = args.x;
        self.y = args.y;
    }

    fn mouse_press(&mut self, _window: &mut GameWindowGLFW, args: &MousePressArgs) {
        match args.button {
            Left => self.tetromino_manager.rotate_tetromino(),
            Right => self.tetromino_manager.generate_next(),
            _ => {}
        }
    }
}

