mod game;
mod snake;
mod food;
mod audio;
mod highscore;

use ggez::{event::run, ContextBuilder, conf::WindowSetup, conf::WindowMode};
use game::Game;

fn main() -> ggez::GameResult {
    let window_setup = WindowSetup::default().title("Snake Game v0.1.0");
    let window_mode = WindowMode::default().dimensions(450.0, 500.0); // Grid + padding for score

    let (ctx, event_loop) = ContextBuilder::new("snake", "michael")
        .add_resource_path("./assets")
        .window_setup(window_setup)
        .window_mode(window_mode)
        .build()?;

    let game = Game::new(&ctx)?;
    run(ctx, event_loop, game)
}
