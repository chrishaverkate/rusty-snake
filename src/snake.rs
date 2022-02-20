use crate::food::*;
use crate::game_settings::*;

use opengl_graphics::GlGraphics;
use piston::input::*;
use std::collections::LinkedList;

#[derive(Clone, PartialEq)]
pub enum Direction {
	Right,
	Left,
	Up,
	Down,
}

pub struct Snake {
	pub body: LinkedList<(i32, i32)>,
	pub dir: Direction,
}

impl Snake {
	pub fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
		// use graphics;

		let squares: Vec<graphics::types::Rectangle> = self
			.body
			.iter()
			.map(|&(x, y)| {
				graphics::rectangle::square(
					(x as f64) * CELL_SIZE,
					(y as f64) * CELL_SIZE,
					CELL_SIZE,
				)
			})
			.collect();

		gl.draw(args.viewport(), |c, gl| {
			let transform = c.transform;
			squares
				.into_iter()
				.for_each(|square| graphics::rectangle(COLOR_SNAKE, square, transform, gl));
		});
	}

	pub fn current_direction(&self) -> Direction {
		return self.dir.clone();
	}

	pub fn change_direction(&mut self, new_direction: &Direction) {
		let last_direction = self.dir.clone();

		self.dir = match new_direction {
			Direction::Up if last_direction != Direction::Down => Direction::Up,
			Direction::Down if last_direction != Direction::Up => Direction::Down,
			Direction::Left if last_direction != Direction::Right => Direction::Left,
			Direction::Right if last_direction != Direction::Left => Direction::Right,
			_ => last_direction,
		};
	}

	pub fn update(&mut self, food: &mut Food) {
		let mut new_head = (*self.body.front().expect("Snake has no body")).clone();

		match self.dir {
			Direction::Right => new_head.0 += 1,
			Direction::Left => new_head.0 -= 1,
			Direction::Up => new_head.1 -= 1,
			Direction::Down => new_head.1 += 1,
		}

		self.body.push_front(new_head);

		// If the snake head touches the the food, grow by not removing the tail
		if food.pos_x == new_head.0 && food.pos_y == new_head.1 {
			food.eaten = true;
			println!("Ate food! Score -> {}", self.body.len());
		} else {
			self.body.pop_back().unwrap();
		}
	}

	pub fn alive(&self) -> bool {
		let head = *self.body.front().expect("Snake has no body");
		let out_of_bounds_side = head.0 < 0 || BOARD_WIDTH <= head.0;
		let out_of_bounds_top_or_bottom = head.1 < 0 || BOARD_HEIGHT <= head.1;

		return !out_of_bounds_side && !out_of_bounds_top_or_bottom && !self.collided_with_self();
	}

	fn collided_with_self(&self) -> bool {
		for position in self.body.iter().skip(1) {
			if position == self.body.front().expect("snake has no body") {
				return true;
			}
		}

		return false;
	}
}
