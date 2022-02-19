extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

use std::collections::LinkedList;
use std::iter::FromIterator;

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
	body: LinkedList<(i32, i32)>,
	dir: Direction,
}

impl Snake {
	fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
		// use graphics;

		let red: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

		let squares: Vec<graphics::types::Rectangle> = self
			.body
			.iter()
			.map(|&(x, y)| graphics::rectangle::square((x * 20) as f64, (y * 20) as f64, 20_f64))
			.collect();

		gl.draw(args.viewport(), |c, gl| {
			let transform = c.transform;
			squares
				.into_iter()
				.for_each(|square| graphics::rectangle(red, square, transform, gl));
		});
	}

	fn update(&mut self) {
		let mut new_head = (*self.body.front().expect("Snake has no body")).clone();

		match self.dir {
			Direction::Right => new_head.0 += 1,
			Direction::Left => new_head.0 -= 1,
			Direction::Up => new_head.1 -= 1,
			Direction::Down => new_head.1 += 1,
		}

		self.body.push_front(new_head);
		self.body.pop_back().unwrap();
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
			body: LinkedList::from_iter((vec![(0, 0), (0, 1)]).into_iter()),
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
