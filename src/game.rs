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
};

use graphics::{
    Context,
    AddRectangle,
    AddColor,
    Draw,
    RelativeTransform2d,
};

pub static BLOCK_SIZE: u32 = 20;

pub struct App {
    gl: Gl,        // OpenGL drawing backend.
    block_size: f64,
    x: f64,
    y: f64,
}

impl Game for App {
    fn render(&mut self, args: &RenderArgs) {
        let context = &Context::abs(args.width as f64, args.height as f64);
        context.rgba(0.0, 0.0, 0.0, 1.0).draw(&mut self.gl);

        context
            .rect(0.0, 0.0, self.block_size * 4.0, self.block_size * 4.0)
            .rgba(1.0, 0.0, 0.0, 0.5)
            .trans(self.x, self.y)
            .draw(&mut self.gl);
    }

    fn update(&mut self, _args: &UpdateArgs) {
        self.y += 1.0;
    }

    fn mouse_move(&mut self, args: &MouseMoveArgs) {
        self.x = args.x;
        self.y = args.y;
    }
}

fn main() {
    // Create a GLFW window.
    let mut window = GameWindowGLFW::new(
        GameWindowSettings {
            title: "My Game".to_string(),
            size: [BLOCK_SIZE * 10, BLOCK_SIZE * 20],
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
        block_size: BLOCK_SIZE as f64,
        x: 0.0,
        y: 0.0,
    };
    app.run(&mut window, &game_iter_settings);
}
