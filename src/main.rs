extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

#[derive(Clone, PartialEq)]
enum Direction {
	Right,
	Left,
	Up,
	Down,
}

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

	fn update(&mut self) {
		self.snake.update();
	}

	fn pressed(&mut self, button: &Button) {
		let last_direction = self.snake.dir.clone();

		self.snake.dir = match button {
			&Button::Keyboard(Key::Up) if last_direction != Direction::Down => Direction::Up,
			&Button::Keyboard(Key::Down) if last_direction != Direction::Up => Direction::Down,
			&Button::Keyboard(Key::Left) if last_direction != Direction::Right => Direction::Left,
			&Button::Keyboard(Key::Right) if last_direction != Direction::Left => Direction::Right,
			_ => last_direction,
		}
	}
}

struct Snake {
	pos_x: i32,
	pos_y: i32,
	dir: Direction,
}

impl Snake {
	fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
		// use graphics;

		let red: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

		let square =
			graphics::rectangle::square((self.pos_x * 20) as f64, (self.pos_y * 20) as f64, 20_f64);

		gl.draw(args.viewport(), |c, gl| {
			let transform = c.transform;
			graphics::rectangle(red, square, transform, gl);
		});
	}

	fn update(&mut self) {
		match self.dir {
			Direction::Right => self.pos_x += 1,
			Direction::Left => self.pos_x -= 1,
			Direction::Up => self.pos_y -= 1,
			Direction::Down => self.pos_y += 1,
		}
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
			pos_x: 0,
			pos_y: 0,
			dir: Direction::Right,
		},
	};

	let mut events = Events::new(EventSettings::new()).ups(2);

	// when there is another event in the window...
	while let Some(e) = events.next(&mut window) {
		// ... if that event is a "Render"...
		if let Some(args) = e.render_args() {
			// ... render the game
			game.render(&args);
		}

		if let Some(args) = e.update_args() {
			game.update();
		}

		if let Some(args) = e.button_args() {
			if args.state == ButtonState::Press {
				game.pressed(&args.button);
			}
		}
	}
}
