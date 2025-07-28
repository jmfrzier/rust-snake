use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect};
use ggez::Context;
use rand::Rng;

use crate::snake::{Position, Snake};

const GRID_SIZE: i32 = 20;
const CELL_SIZE: f32 = 20.0;

pub struct Food {
    pub position: Position,
}

impl Food {
    pub fn new() -> Self {
        let mut food = Self { position: Position { x: 0, y: 0 } };
        food.respawn_safe(&Snake::new());
        food
    }

    pub fn respawn(&mut self) {
        let mut rng = rand::thread_rng();
        self.position.x = rng.gen_range(0..GRID_SIZE);
        self.position.y = rng.gen_range(0..GRID_SIZE);
    }

    // NEW: Safe respawn that avoids snake body
    pub fn respawn_safe(&mut self, snake: &Snake) {
        let mut rng = rand::thread_rng();
        loop {
            self.position.x = rng.gen_range(0..GRID_SIZE);
            self.position.y = rng.gen_range(0..GRID_SIZE);

            // Make sure food doesn't spawn on snake
            if !snake.contains_position(self.position) {
                break;
            }
        }
    }

    pub fn draw(&self, ctx: &Context, canvas: &mut Canvas) -> ggez::GameResult {
        let rect = Rect::new_i32(
            self.position.x * CELL_SIZE as i32,
            self.position.y * CELL_SIZE as i32,
            CELL_SIZE as i32,
            CELL_SIZE as i32,
        );
        let mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), rect, Color::RED)?;
        canvas.draw(&mesh, DrawParam::default());
        Ok(())
    }
}
