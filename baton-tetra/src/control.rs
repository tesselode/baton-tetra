use tetra::Context;

use crate::source::{InputKind, InputSource};

pub trait ControlKindTrait: Eq + std::hash::Hash + Sized + 'static {
	fn kinds() -> &'static [Self];
}

/// Input data for an in-game control.
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
		sources: &[InputSource],
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

	/// Returns the value of the control without deadzone applied.
	pub fn raw_value(&self) -> f32 {
		self.raw_value
	}

	/// Returns the value of the control with deadzone applied.
	pub fn value(&self) -> f32 {
		self.value
	}

	/// Returns true if the control is currently held down,
	/// i.e. its value is greater than or equal to the deadzone.
	pub fn down(&self) -> bool {
		self.down
	}

	/// Returns true if the control started being held down
	/// this frame.
	pub fn pressed(&self) -> bool {
		self.pressed
	}

	/// Returns true if the control stopped being held down
	/// this frame.
	pub fn released(&self) -> bool {
		self.released
	}
}

impl Default for Control {
	fn default() -> Self {
		Self::new()
	}
}
