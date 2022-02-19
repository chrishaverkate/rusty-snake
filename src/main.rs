extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

// State of game
struct Game {
	gl: GlGraphics, // The graphic window
	snake: Snake,
}

//Implementation related to Game struct
impl Game {
	fn render(&mut self, arg: &RenderArgs) {
		// use graphics;

		// RGBA
		let green: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

		self.gl.draw(arg.viewport(), |_c, gl| {
			graphics::clear(green, gl);
		});

		self.snake.render(&mut self.gl, arg);
	}
}

struct Snake {
	pos_x: i32,
	pos_y: i32,
}

impl Snake {
	fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
		// use graphics;

		let red: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

		let square = graphics::rectangle::square(self.pos_x as f64, self.pos_y as f64, 20_f64);

		gl.draw(args.viewport(), |c, gl| {
			let transform = c.transform;
			graphics::rectangle(red, square, transform, gl);
		});
	}
}

fn main() {
	let opengl = OpenGL::V3_2;

	// Create an Glutin window.
	let mut window: GlutinWindow = WindowSettings::new("RustySnake", [200, 200])
		.graphics_api(opengl)
		.exit_on_esc(true)
		.build()
		.unwrap();

	// Create a new game and run
	let mut game = Game {
		gl: GlGraphics::new(opengl),
		snake: Snake {
			pos_x: 20,
			pos_y: 20,
		},
	};

	let mut events = Events::new(EventSettings::new());

	// when there is another event in the window...
	while let Some(e) = events.next(&mut window) {
		// ... if that event is a "Render"...
		if let Some(args) = e.render_args() {
			// ... render the game
			game.render(&args);
		}

		// if let Some(args) = e.update_args() {
		// 	 app.update(&args);
		// }
	}
}
