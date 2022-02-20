mod food;
mod game_settings;
mod snake;

use crate::food::*;
use crate::game_settings::*;
use crate::snake::{Direction, Snake};

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

		let scoreboard = graphics::rectangle::rectangle_by_corners(
			0.0,
			(BOARD_HEIGHT * CELL_SIZE) as f64,
			(FRAME_WIDTH * CELL_SIZE) as f64,
			(FRAME_HEIGHT * CELL_SIZE) as f64,
		);

		self.gl.draw(arg.viewport(), |c, gl| {
			let transform = c.transform;
			graphics::rectangle(COLOR_METADATA, scoreboard, transform, gl)
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

fn main() {
	let opengl = OpenGL::V3_2;

	let width = FRAME_WIDTH * CELL_SIZE;
	let height = FRAME_HEIGHT * CELL_SIZE;

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

	let mut events = Events::new(EventSettings::new()).ups(FRAMES_PER_SECOND);

	// when there is another event in the window...
	while let Some(e) = events.next(&mut window) {
		// ... if that event is a "Render"...
		if let Some(args) = e.render_args() {
			// ... render the game
			game.render(&args);
		}

		if let Some(_args) = e.update_args() {
			game.update();
		}

		if let Some(args) = e.button_args() {
			if args.state == ButtonState::Press {
				game.pressed(&args.button);
			}
		}

		if !game.snake.alive() {
			println!("DEAD!");
			break;
		}
	}
}
