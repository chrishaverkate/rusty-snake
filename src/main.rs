extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

use rand::Rng;
use std::collections::LinkedList;
use std::iter::FromIterator;

const BOARD_WIDTH: i32 = 40; // in number of cells
const BOARD_HEIGHT: i32 = 40; // in number of cells
const CELL_SIZE: i32 = 20; // in pixels
const FREQ_UPDATE: i32 = 20; // The number of loops that must pass before the game updates. Controls the "speed" of the game.
const FRAMES_PER_SECOND: u64 = 30;

const COLOR_BG: [f32; 4] = [0.0, 0.80, 0.40, 1.0];
const COLOR_SNAKE: [f32; 4] = [0.9, 0.10, 0.10, 1.0];
const COLOR_FOOD: [f32; 4] = [0.50, 0.30, 0.0, 1.0];

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
	food: Food,
}

//Implementation related to Game struct
impl Game {
	fn render(&mut self, arg: &RenderArgs) {
		// use graphics;

		self.gl.draw(arg.viewport(), |_c, gl| {
			graphics::clear(COLOR_BG, gl);
		});

		self.snake.render(&mut self.gl, arg);
		self.food.render(&mut self.gl, arg);
	}

	fn update(&mut self) {
		self.snake.update(&mut self.food);
		self.food.update();
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

	fn update(&mut self, food: &mut Food) {
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
			println!("Ate food!");
		} else {
			self.body.pop_back().unwrap();
		}
	}
}

struct Food {
	pos_x: i32,
	pos_y: i32,
	eaten: bool,
}

impl Food {
	fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
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

	fn update(&mut self) {
		if self.eaten {
			self.eaten = false;
			self.pos_x = rand::thread_rng().gen_range(0..BOARD_WIDTH);
			self.pos_y = rand::thread_rng().gen_range(0..BOARD_HEIGHT);
			println!("Place new food {}, {}", self.pos_x, self.pos_y);
		}
	}
}

fn main() {
	let opengl = OpenGL::V3_2;

	let width = BOARD_WIDTH * CELL_SIZE;
	let height = BOARD_HEIGHT * CELL_SIZE;

	// Create an Glutin window.
	let mut window: GlutinWindow = WindowSettings::new("RustySnake", [width as f64, height as f64])
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
		food: Food {
			pos_x: BOARD_WIDTH / 2,
			pos_y: BOARD_HEIGHT / 2,
			eaten: false,
		},
	};

	let mut events = Events::new(EventSettings::new()); //.ups(FRAMES_PER_SECOND);
	let update_freq = FREQ_UPDATE;
	let mut loop_count = 0;

	// when there is another event in the window...
	while let Some(e) = events.next(&mut window) {
		loop_count += 1;

		// ... if that event is a "Render"...
		if let Some(args) = e.render_args() {
			// ... render the game
			game.render(&args);
		}

		if loop_count == update_freq {
			loop_count = 0;
			if let Some(_args) = e.update_args() {
				game.update();
			}
		}

		if let Some(args) = e.button_args() {
			if args.state == ButtonState::Press {
				game.pressed(&args.button);
			}
		}
	}
}
