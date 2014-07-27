extern crate graphics;
extern crate piston;
extern crate glfw_game_window;
extern crate opengl_graphics;

use glfw_game_window::GameWindowGLFW;
use piston::{
    Game,
    GameIteratorSettings,
    GameWindowSettings,
};

use game::{
    App,
    CELL_SIZE,
    CELL_WIDTH,
    CELL_HEIGHT
};

mod game;
mod block;

fn main() {
    // Create a GLFW window.
    let mut window = GameWindowGLFW::new(
        GameWindowSettings {
            title: "My Game".to_string(),
            size: [game::CELL_SIZE * game::CELL_WIDTH, game::CELL_SIZE * game::CELL_HEIGHT],
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
    let game: &mut Game<GameWindowGLFW> = &mut App::new();
    game.run(&mut window, &game_iter_settings);
}
