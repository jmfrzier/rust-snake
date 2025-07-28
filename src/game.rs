use ggez::{
    event::EventHandler,
    graphics::{Canvas, Color, DrawParam, Text, TextFragment},
    input::keyboard::{KeyCode, KeyInput},
    Context, GameResult,
};
use ggez::glam::Vec2;

use crate::{audio, food::Food, snake::{Direction, Snake}};

pub struct Game {
    snake: Snake,
    food: Food,
    score: usize,
    game_over: bool,
    timer: f32,
}

impl Game {
    pub fn new(_ctx: &Context) -> GameResult<Self> {
        // IMPROVED: Handle audio initialization error gracefully
        if let Err(e) = audio::init_audio() {
            eprintln!("Warning: Audio initialization failed: {}", e);
        }

        Ok(Self {
            snake: Snake::new(),
            food: Food::new(),
            score: 0,
            game_over: false,
            timer: 0.0,
        })
    }

    // NEW: Draw grid lines to show boundaries
    fn draw_grid(&self, ctx: &Context, canvas: &mut Canvas) -> GameResult {
        use ggez::graphics::Mesh;

        const GRID_SIZE: i32 = 20;
        const CELL_SIZE: f32 = 20.0;

        // Define colors as variables instead of constants
        let grid_color = Color::from_rgb(40, 40, 40); // Dark gray
        let border_color = Color::from_rgb(100, 100, 100); // Light gray

        // Draw vertical grid lines
        for x in 0..=GRID_SIZE {
            let x_pos = x as f32 * CELL_SIZE;
            let line = Mesh::new_line(
                ctx,
                &[
                    Vec2::new(x_pos, 0.0),
                    Vec2::new(x_pos, GRID_SIZE as f32 * CELL_SIZE),
                ],
                1.0,
                grid_color,
            )?;
            canvas.draw(&line, DrawParam::default());
        }

        // Draw horizontal grid lines
        for y in 0..=GRID_SIZE {
            let y_pos = y as f32 * CELL_SIZE;
            let line = Mesh::new_line(
                ctx,
                &[
                    Vec2::new(0.0, y_pos),
                    Vec2::new(GRID_SIZE as f32 * CELL_SIZE, y_pos),
                ],
                1.0,
                grid_color,
            )?;
            canvas.draw(&line, DrawParam::default());
        }

        // Draw border (thicker lines around the edge)
        let border_lines = vec![
            // Top border
            ([Vec2::new(0.0, 0.0), Vec2::new(GRID_SIZE as f32 * CELL_SIZE, 0.0)], 3.0),
            // Bottom border
            ([Vec2::new(0.0, GRID_SIZE as f32 * CELL_SIZE), Vec2::new(GRID_SIZE as f32 * CELL_SIZE, GRID_SIZE as f32 * CELL_SIZE)], 3.0),
            // Left border
            ([Vec2::new(0.0, 0.0), Vec2::new(0.0, GRID_SIZE as f32 * CELL_SIZE)], 3.0),
            // Right border
            ([Vec2::new(GRID_SIZE as f32 * CELL_SIZE, 0.0), Vec2::new(GRID_SIZE as f32 * CELL_SIZE, GRID_SIZE as f32 * CELL_SIZE)], 3.0),
        ];

        for (points, width) in border_lines {
            let border = Mesh::new_line(ctx, &points, width, border_color)?;
            canvas.draw(&border, DrawParam::default());
        }

        Ok(())
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const UPDATE_INTERVAL: f32 = 0.15;

        if self.game_over {
            return Ok(());
        }

        self.timer += ctx.time.delta().as_secs_f32();

        if self.timer >= UPDATE_INTERVAL {
            self.timer = 0.0;
            self.snake.update();

            // Check food collision
            if self.snake.head_position() == self.food.position {
                self.snake.grow();
                self.food.respawn_safe(&self.snake); // IMPROVED: Use safe respawn
                self.score += 1;
                audio::play_eat_sound();
            }

            // Check self collision or wall collision
            if self.snake.check_self_collision() || self.snake.check_wall_collision() {
                self.game_over = true;
                audio::play_die_sound();
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);

        // Draw grid lines
        self.draw_grid(ctx, &mut canvas)?;

        self.snake.draw(ctx, &mut canvas)?;
        self.food.draw(ctx, &mut canvas)?;

        // Draw score
        let score_text = Text::new(TextFragment::new(format!("Score: {}", self.score)).scale(24.0));
        canvas.draw(&score_text, DrawParam::default().dest(Vec2::new(10.0, 10.0)));

        if self.game_over {
            let screen_coords = ctx.gfx.drawable_size();

            // First line: "Game Over!"
            let game_over_text = Text::new(TextFragment::new("Game Over!").scale(28.0));
            let dest1 = Vec2::new(screen_coords.0 / 2.0 - 60.0, screen_coords.1 / 2.0 + 30.0);
            canvas.draw(&game_over_text, DrawParam::default().dest(dest1));

            // Second line: "Press R to restart"
            let restart_text = Text::new(TextFragment::new("Press R to restart").scale(20.0));
            let dest2 = Vec2::new(screen_coords.0 / 2.0 - 75.0, screen_coords.1 / 2.0 + 65.0);
            canvas.draw(&restart_text, DrawParam::default().dest(dest2));
        }

        canvas.finish(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        input: KeyInput,
        _repeated: bool,
    ) -> GameResult {
        if let Some(keycode) = input.keycode {
            use KeyCode::*;

            // NEW: Restart functionality
            if keycode == R && self.game_over {
                *self = Self::new(_ctx)?;
                return Ok(());
            }

            let dir = match keycode {
                Up => Some(Direction::Up),
                Down => Some(Direction::Down),
                Left => Some(Direction::Left),
                Right => Some(Direction::Right),
                _ => None,
            };

            if let Some(d) = dir {
                self.snake.change_direction(d);
            }
        }

        Ok(())
    }
}
