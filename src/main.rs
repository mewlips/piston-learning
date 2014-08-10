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
    BLOCK_SIZE,
    BLOCK_WIDTH,
    BLOCK_HEIGHT,
};

mod game;
mod block;
mod space;
mod tetromino;

fn main() {
    // Create a GLFW window.
    let mut window = GameWindowGLFW::new(
        GameWindowSettings {
            title: "My Game".to_string(),
            //size: [game.getBoardWidth(), game.getBoardHeight()],
            size: [(game::BLOCK_SIZE * game::BLOCK_WIDTH) as u32,
                   (game::BLOCK_SIZE * game::BLOCK_HEIGHT) as u32],
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
