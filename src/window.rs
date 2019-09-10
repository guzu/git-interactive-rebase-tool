use pancurses::Input;

use crate::color::Color;
use crate::config::Config;
use std::cell::RefCell;

use pancurses;

const COLOR_TABLE: [i16; 8] = [
	pancurses::COLOR_BLACK, // the default foreground color must be the first (see #77)
	pancurses::COLOR_BLUE,
	pancurses::COLOR_CYAN,
	pancurses::COLOR_GREEN,
	pancurses::COLOR_MAGENTA,
	pancurses::COLOR_RED,
	pancurses::COLOR_YELLOW,
	pancurses::COLOR_WHITE,
];

#[derive(Clone, Copy, Debug)]
pub enum WindowColor {
	ActionBreak,
	ActionDrop,
	ActionEdit,
	ActionExec,
	ActionFixup,
	ActionPick,
	ActionReword,
	ActionSquash,
	DiffAddColor,
	DiffRemoveColor,
	DiffChangeColor,
	Foreground,
	IndicatorColor,
}

pub struct Window<'w> {
	config: &'w Config,
	pub window: pancurses::Window,
	height: RefCell<i32>,
	width: RefCell<i32>,
}

impl<'w> Window<'w> {
	pub fn getch(&self) -> Option<Input> {
		let input = self.window.getch();

		if let Some(Input::KeyResize) = input {
			pancurses::resize_term(0, 0);
			self.height.replace(self.window.get_max_y());
			self.width.replace(self.window.get_max_x());
		}
		input
	}

	pub fn get_window_size(&self) -> (i32, i32) {
		(*self.width.borrow(), *self.height.borrow())
	}

	/// Leaves curses mode, runs the specified callback, and re-enables curses.
	pub fn leave_temporarily<F, T>(callback: F) -> T
	where F: FnOnce() -> T {
		pancurses::def_prog_mode();
		pancurses::endwin();
		let rv = callback();
		pancurses::reset_prog_mode();
		rv
	}
}
