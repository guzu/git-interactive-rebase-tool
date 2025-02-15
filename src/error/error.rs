use crate::git_interactive::GitInteractive;
use crate::input::{Input, InputHandler};
use crate::process::{HandleInputResult, HandleInputResultBuilder, ProcessModule, State};
use crate::view::View;

pub struct Error {
	error_message: String,
	return_state: State,
}

impl ProcessModule for Error {
	fn activate(&mut self, state: State, _git_interactive: &GitInteractive) {
		if let State::Error { message, return_state } = state {
			self.error_message = message;
			self.return_state = *return_state;
		}
		else {
			panic!("Help module activated when not expected");
		}
	}

	fn deactivate(&mut self) {
		self.error_message.clear();
	}

	fn handle_input(
		&mut self,
		input_handler: &InputHandler,
		_git_interactive: &mut GitInteractive,
		_view: &View,
	) -> HandleInputResult
	{
		let input = input_handler.get_input();
		let mut result = HandleInputResultBuilder::new(input);
		match input {
			Input::Resize => {},
			_ => {
				result = result.state(self.return_state.clone());
			},
		}
		result.build()
	}

	fn render(&self, view: &View, _git_interactive: &GitInteractive) {
		view.draw_error(self.error_message.as_str());
	}
}

impl Error {
	pub fn new() -> Self {
		Self {
			error_message: String::from(""),
			return_state: State::List(false),
		}
	}
}
