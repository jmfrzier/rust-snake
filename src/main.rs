mod game;
mod snake;
mod food;
mod audio;

use ggez::{event::run, ContextBuilder, conf::WindowSetup};
use game::Game;

fn main() -> ggez::GameResult {
    let window_setup = WindowSetup::default().title("Snake Game v0.1.0");

    let (ctx, event_loop) = ContextBuilder::new("snake", "michael")
        .add_resource_path("./assets")
        .window_setup(window_setup)
        .build()?;

    let game = Game::new(&ctx)?;
    run(ctx, event_loop, game)
}
