use tetra::{
	input::{GamepadAxis, GamepadButton, Key},
	Context,
};

/// Categories of input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputKind {
	Keyboard,
	Gamepad,
}

/// A direction an axis can be moved in.
#[cfg_attr(
	feature = "serde_support",
	derive(serde::Serialize, serde::Deserialize)
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AxisDirection {
	/// The negative direction, i.e. left or up on a gamepad
	/// stick.
	Negative,
	/// The positive direction, i.e. right or down on a gamepad
	/// stick.
	Positive,
}

impl AxisDirection {
	fn as_f32(self) -> f32 {
		match self {
			AxisDirection::Negative => -1.0,
			AxisDirection::Positive => 1.0,
		}
	}
}

/// A source of data from a hardware input device.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
	feature = "serde_support",
	derive(serde::Serialize, serde::Deserialize)
)]
pub enum InputSource {
	/// A keyboard key.
	Key(Key),
	/// A button on a gamepad.
	Button(GamepadButton),
	/// A gamepad axis filtered to movement in one direction.
	Axis(GamepadAxis, AxisDirection),
}

impl InputSource {
	pub(crate) fn get(&self, ctx: &Context, gamepad_id: Option<usize>) -> f32 {
		match self {
			InputSource::Key(key) => {
				if tetra::input::is_key_down(ctx, *key) {
					1.0
				} else {
					0.0
				}
			}
			InputSource::Button(button) => {
				if let Some(gamepad_id) = gamepad_id {
					if tetra::input::is_gamepad_button_down(ctx, gamepad_id, *button) {
						1.0
					} else {
						0.0
					}
				} else {
					0.0
				}
			}
			InputSource::Axis(axis, direction) => {
				if let Some(gamepad_id) = gamepad_id {
					let axis_position =
						tetra::input::get_gamepad_axis_position(ctx, gamepad_id, *axis);
					(axis_position * direction.as_f32()).max(0.0)
				} else {
					0.0
				}
			}
		}
	}

	pub(crate) fn kind(&self) -> InputKind {
		match self {
			InputSource::Key(_) => InputKind::Keyboard,
			InputSource::Button(_) => InputKind::Gamepad,
			InputSource::Axis(_, _) => InputKind::Gamepad,
		}
	}
}

impl From<Key> for InputSource {
	fn from(key: Key) -> Self {
		Self::Key(key)
	}
}

impl From<GamepadButton> for InputSource {
	fn from(button: GamepadButton) -> Self {
		Self::Button(button)
	}
}

impl From<(GamepadAxis, AxisDirection)> for InputSource {
	fn from(axis_and_direction: (GamepadAxis, AxisDirection)) -> Self {
		Self::Axis(axis_and_direction.0, axis_and_direction.1)
	}
}
