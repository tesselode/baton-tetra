use tetra::Context;

use crate::source::{InputKind, InputSource};

pub struct Control {
	raw_value: f32,
	value: f32,
	down: bool,
	pressed: bool,
	released: bool,
}

impl Control {
	pub(crate) fn new() -> Self {
		Self {
			raw_value: 0.0,
			value: 0.0,
			down: false,
			pressed: false,
			released: false,
		}
	}

	pub(crate) fn update(
		&mut self,
		ctx: &Context,
		gamepad_id: Option<usize>,
		sources: &Vec<InputSource>,
		deadzone: f32,
		active_input_kind: Option<InputKind>,
	) {
		self.raw_value = 0.0;
		if let Some(input_kind) = active_input_kind {
			for source in sources {
				if source.kind() == input_kind {
					self.raw_value += source.get(ctx, gamepad_id);
					if self.raw_value >= 1.0 {
						self.raw_value = 1.0;
						break;
					}
				}
			}
		}
		self.value = if self.raw_value >= deadzone {
			self.raw_value
		} else {
			0.0
		};
		let down_previous = self.down;
		self.down = self.raw_value >= deadzone;
		self.pressed = self.down && !down_previous;
		self.released = down_previous && !self.down;
	}

	pub fn raw_value(&self) -> f32 {
		self.raw_value
	}

	pub fn value(&self) -> f32 {
		self.value
	}

	pub fn down(&self) -> bool {
		self.down
	}

	pub fn pressed(&self) -> bool {
		self.pressed
	}

	pub fn released(&self) -> bool {
		self.released
	}
}

impl Default for Control {
	fn default() -> Self {
		Self::new()
	}
}
