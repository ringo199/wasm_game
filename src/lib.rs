
use std::usize;

use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen(module="/www/utils/random.js")]
extern "C" {
    fn random(max: usize) -> usize;
}

#[wasm_bindgen]
#[derive(PartialEq, Copy, Clone)]
pub enum GameStatus {
    Won,
    Lost,
    Played,
}

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(PartialEq, Copy, Clone)]
pub struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

impl Snake {
    fn new (spawn_index: usize, size: usize) -> Self {
        let mut body = Vec::new();
        for i in 0..size {
            body.push(SnakeCell(spawn_index - i))
        }
        Self {
            body,
            direction: Direction::Down
        }
    }
}

#[wasm_bindgen]
pub struct World {
    width: usize,
    size: usize,
    reward_cell: Option<usize>,
    snake: Snake,
    next_cell: Option<SnakeCell>,
    status: Option<GameStatus>,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_index: usize) -> Self {
        let size = width * width;
        let snake = Snake::new(snake_index, 3);
        Self {
            width,
            size: size,
            reward_cell: Some(World::gen_reward_cell(size, &snake.body)),
            snake: snake,
            next_cell: None,
            status: None,
        }
    }

    fn gen_reward_cell(max: usize, snake_body: &Vec<SnakeCell>) -> usize {
        let mut reward_cell;
        loop {
            reward_cell = random(max);
            if !snake_body.contains(&SnakeCell(reward_cell)) {
                break;
            }
        }
        reward_cell
    }

    pub fn start_game(&mut self) {
        self.status = Some(GameStatus::Played);
    }

    pub fn game_status(&self) -> Option<GameStatus> {
        self.status
    }

    pub fn game_status_info(&self) -> String {
        match self.status {
            Some(GameStatus::Won) => "You Won!".to_string(),
            Some(GameStatus::Lost) => "You Lost!".to_string(),
            Some(GameStatus::Played) => "You Playing!".to_string(),
            None => "None!".to_string(),
        }
    }

    pub fn reward_cell(&self) -> Option<usize> {
        self.reward_cell
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn snake_head_index(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn change_snake_direction(&mut self, direction: Direction) {
        let next_cell = self.gen_next_snake_cell(&direction);
        if self.snake.body[1].0 == next_cell.0 {
            return;
        }
        self.snake.direction = direction;
    }

    pub fn snake_cells(&self) -> *const SnakeCell {
        self.snake.body.as_ptr()
    }

    pub fn snake_length(&self) -> usize {
        self.snake.body.len()
    }

    pub fn update(&mut self) {
        let temp = self.snake.body.clone();

        match self.next_cell {
            Some(cell) => {
                self.snake.body[0] = cell;
                self.next_cell = None;
            },
            None => {
                self.snake.body[0] = self.gen_next_snake_cell(&self.snake.direction)
            }
        }

        let len = self.snake.body.len();
        for i in 1..len {
            self.snake.body[i] = SnakeCell(temp[i - 1].0);
        }
        if self.snake.body[1..len].contains(&self.snake.body[0]) {
            self.status = Some(GameStatus::Lost)
        }

        if self.reward_cell == Some(self.snake_head_index()) {
            if self.snake_length() < self.size {
                self.reward_cell = Some(World::gen_reward_cell(self.size, &self.snake.body));
            } else {
                self.reward_cell = None;
                self.status = Some(GameStatus::Won);
            }

            self.snake.body.push(SnakeCell(self.snake.body[1].0))
        }
    }

    fn gen_next_snake_cell(&self, direction: &Direction) -> SnakeCell {
        let snake_index = self.snake_head_index();
        let row = snake_index / self.width;
        return match direction {
            Direction::Up => {
                let border_hold = snake_index - row * self.width;
                if snake_index == border_hold {
                    SnakeCell((self.size - self.width) + border_hold)
                } else {
                    SnakeCell(snake_index - self.width)
                }
            }
            Direction::Down => {
                let border_hold = snake_index + ((self.width - row) * self.width);
                if snake_index + self.width == border_hold {
                    SnakeCell(border_hold - (row + 1) * self.width)
                } else {
                    SnakeCell(snake_index + self.width)
                }
            }
            Direction::Left => {
                let border_hold = row * self.width;
                if snake_index == border_hold {
                    SnakeCell(border_hold + self.width - 1)
                } else {
                    SnakeCell(snake_index - 1)
                }
            }
            Direction::Right => {
                let border_hold = (row + 1) * self.width;
                if snake_index + 1 == border_hold {
                    SnakeCell(border_hold - self.width)
                } else {
                    SnakeCell(snake_index + 1)
                }
            }
        }
    }

    fn set_snake_head(&mut self, index: usize) {
        self.snake.body[0].0 = index
    }

    fn index_to_cell(&self, index: usize) -> (usize, usize) {
        (index / self.width, index % self.width)
    }

    fn cell_to_index(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }
}
