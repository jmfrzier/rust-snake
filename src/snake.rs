use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect};
use ggez::Context;

const GRID_SIZE: i32 = 20;
const CELL_SIZE: f32 = 20.0;

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub struct Snake {
    body: Vec<Position>,
    dir: Direction,
    grow: bool,
}

impl Snake {
    pub fn new() -> Self {
        let start = Position { x: 10, y: 10 };
        Self {
            body: vec![start],
            dir: Direction::Right,
            grow: false,
        }
    }

    pub fn head_position(&self) -> Position {
        self.body[0]
    }

    pub fn update(&mut self) {
        let mut new_head = self.head_position();

        match self.dir {
            Direction::Up => new_head.y -= 1,
            Direction::Down => new_head.y += 1,
            Direction::Left => new_head.x -= 1,
            Direction::Right => new_head.x += 1,
        }

        // Don't wrap - let wall collision be handled in game.rs
        self.body.insert(0, new_head);
        if !self.grow {
            self.body.pop();
        } else {
            self.grow = false;
        }
    }

    pub fn grow(&mut self) {
        self.grow = true;
    }

    pub fn change_direction(&mut self, dir: Direction) {
        use Direction::*;
        // Prevent reversing direction
        if (self.dir == Up && dir != Down)
            || (self.dir == Down && dir != Up)
            || (self.dir == Left && dir != Right)
            || (self.dir == Right && dir != Left)
        {
            self.dir = dir;
        }
    }

    pub fn check_self_collision(&self) -> bool {
        // FIXED: Check if body has more than 1 segment before checking collision
        if self.body.len() <= 1 {
            return false;
        }

        let head = self.head_position();
        self.body[1..].contains(&head)
    }

    // NEW: Check wall collision
    pub fn check_wall_collision(&self) -> bool {
        let head = self.head_position();
        head.x < 0 || head.x >= GRID_SIZE || head.y < 0 || head.y >= GRID_SIZE
    }

    // Helper method for food spawning
    pub fn contains_position(&self, pos: Position) -> bool {
        self.body.contains(&pos)
    }

    pub fn draw(&self, ctx: &Context, canvas: &mut Canvas) -> ggez::GameResult {
        for (i, segment) in self.body.iter().enumerate() {
            let rect = Rect::new_i32(
                segment.x * CELL_SIZE as i32,
                segment.y * CELL_SIZE as i32,
                CELL_SIZE as i32,
                CELL_SIZE as i32,
            );

            // Make head slightly different color
            let color = if i == 0 { Color::from_rgb(0, 255, 100) } else { Color::GREEN };
            let mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), rect, color)?;
            canvas.draw(&mesh, DrawParam::default());
        }
        Ok(())
    }
}
