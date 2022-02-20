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
				graphics::rectangle::square((x * CELL_SIZE) as f64, (y * CELL_SIZE) as f64, 20_f64)
			})
			.collect();

		gl.draw(args.viewport(), |c, gl| {
			let transform = c.transform;
			squares
				.into_iter()
				.for_each(|square| graphics::rectangle(COLOR_SNAKE, square, transform, gl));
		});
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
		return !out_of_bounds_side && !out_of_bounds_top_or_bottom;
	}
}
