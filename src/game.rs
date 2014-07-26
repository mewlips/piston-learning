extern crate graphics;
extern crate piston;
extern crate glfw_game_window;
extern crate opengl_graphics;

use glfw_game_window::GameWindowGLFW;
use opengl_graphics::Gl;

use piston::{
    Game,
    GameWindowSettings,
    GameIteratorSettings,
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
    BlockManager,
};

mod block;

pub static CELL_SIZE:   u32 = 40;
pub static CELL_WIDTH:  u32 = 10;
pub static CELL_HEIGHT: u32 = 20;

pub struct App {
    gl: Gl,        // OpenGL drawing backend.
    block_manager: BlockManager,
    x: f64,
    y: f64,
}

impl App {
    fn draw_block(&mut self, context: &Context) {
        let block = self.block_manager.get_curr_block();

        for y in range(0u32,4) {
            for x in range(0u32,4) {
                if *block.get((y*4 + x) as uint) {
                    context
                        .rect((x * CELL_SIZE) as f64, (y * CELL_SIZE) as f64,
                              CELL_SIZE as f64, CELL_SIZE as f64)
                        .rgba(1.0, 0.0, 0.0, 1.0)
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
        self.draw_block(context);
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
            Left => self.block_manager.rotate_block(),
            Right => self.block_manager.get_next_block(),
            _ => {}
        }
    }
}

fn main() {
    // Create a GLFW window.
    let mut window = GameWindowGLFW::new(
        GameWindowSettings {
            title: "My Game".to_string(),
            size: [CELL_SIZE * CELL_WIDTH, CELL_SIZE * CELL_HEIGHT],
            fullscreen: false,
            exit_on_esc: true
        }
    );

    // Some settings for how the game should be run.
    let game_iter_settings = GameIteratorSettings {
        updates_per_second: 60,
        max_frames_per_second: 60
    };

    // Create a new game and run it.
    let mut app = App {
        gl: Gl::new(),
        block_manager: BlockManager::new(),
        x: 0.0,
        y: 0.0,
    };
    app.run(&mut window, &game_iter_settings);
}
