use crate::game_settings::*;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use opengl_graphics::GlGraphics;
use piston::input::*;
use rand::Rng;

pub struct Food {
	pub pos_x: i32,
	pub pos_y: i32,
	pub eaten: bool,
}

impl Food {
	pub fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
		let offset = (CELL_SIZE - FOOD_SIZE) / 2.0;

		let square = graphics::rectangle::square(
			(self.pos_x as f64) * CELL_SIZE + offset,
			(self.pos_y as f64) * CELL_SIZE + offset,
			FOOD_SIZE,
		);

		gl.draw(args.viewport(), |c, gl| {
			let transform = c.transform;
			graphics::rectangle(COLOR_FOOD, square, transform, gl)
		});
	}

	pub fn update(&mut self) {
		if self.eaten {
			self.eaten = false;
			self.pos_x = rand::thread_rng().gen_range(0..BOARD_WIDTH);
			self.pos_y = rand::thread_rng().gen_range(0..BOARD_HEIGHT);
			println!("Place new food {}, {}", self.pos_x, self.pos_y);
		}
	}
}
