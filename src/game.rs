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

use tetromino::{
    TetrominoManager,
};

pub static CELL_SIZE:   uint = 40;
pub static CELL_WIDTH:  uint = 10;
pub static CELL_HEIGHT: uint = 20;

pub struct App {
    gl: Gl,        // OpenGL drawing backend.
    tetromino_manager: TetrominoManager,
    x: f64,
    y: f64,
}

impl App {
    pub fn new() -> App {
        App {
            gl: Gl::new(),
            tetromino_manager: TetrominoManager::new(),
            x: 0.0,
            y: 0.0,
        }
    }

    fn draw_tetromino(&mut self, context: &Context) {
        let width = self.tetromino_manager.get_width();
        let height = self.tetromino_manager.get_height();
        let (tetromino, v) = self.tetromino_manager.get_curr_tetromino();
        let blocks = &tetromino.blocks[v];
        let (r, g, b, a) = tetromino.color;

        for y in range(0u, height) {
            for x in range(0u, width) {
                if blocks[y][x] {
                    context
                        .rect((x * CELL_SIZE) as f64, (y * CELL_SIZE) as f64,
                              CELL_SIZE as f64, CELL_SIZE as f64)
                        .rgba(r, g, b, a)
                        .trans(self.x, self.y)
                        .draw(&mut self.gl);
                }
            }
        }
    }
}

impl Game<GameWindowGLFW> for App {
    fn render(&mut self, _window: &mut GameWindowGLFW, args: &RenderArgs) {
        let context = &Context::abs(args.width as f64, args.height as f64);
        context.rgba(0.0, 0.0, 0.0, 1.0).draw(&mut self.gl);
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

