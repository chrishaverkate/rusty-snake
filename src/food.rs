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
		// use graphics;

		let square = graphics::rectangle::square(
			(self.pos_x * CELL_SIZE) as f64,
			(self.pos_y * CELL_SIZE) as f64,
			20_f64,
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
