/*!

# Baton

**Baton** is a framework-agnostic input abstraction for games with keyboard and/or
gamepad controls. It combines individual "input sources" (such as keys and
gamepad triggers) into game-specific controls, using opinionated defaults to
determine the final state of each control.

## Example

```
use std::collections::HashMap;
use baton::{DeadzoneShape, GamepadInput, InputConfig, Key, PlayerInput};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, baton::ControlKind)]
enum ControlKind {
	Left,
	Right,
	Up,
	Down,
	Jump,
	Run,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, baton::StickKind)]
#[control_kind(ControlKind)]
enum StickKind {
	#[controls(Left, Right, Up, Down)]
	Move,
}

let mut input = PlayerInput::new(InputConfig {
	control_mapping: {
		let mut control_mapping = HashMap::new();
		control_mapping.insert(
			ControlKind::Left,
			vec![Key::A.into(), GamepadInput::LeftStickLeft.into()],
		);
		control_mapping.insert(
			ControlKind::Right,
			vec![Key::D.into(), GamepadInput::LeftStickRight.into()],
		);
		control_mapping.insert(
			ControlKind::Up,
			vec![Key::W.into(), GamepadInput::LeftStickUp.into()],
		);
		control_mapping.insert(
			ControlKind::Down,
			vec![Key::S.into(), GamepadInput::LeftStickDown.into()],
		);
		control_mapping.insert(
			ControlKind::Jump,
			vec![Key::X.into(), GamepadInput::A.into()],
		);
		control_mapping.insert(
			ControlKind::Run,
			vec![Key::Z.into(), GamepadInput::B.into()],
		);
		control_mapping
	},
	deadzone: 0.25,
	deadzone_shape: DeadzoneShape::Circle,
});

// in your update code:
input.update(baton_tetra::InputProvider(ctx));
let (move_x, move_y) = input.stick(StickKind::Move).value();
if input.control(ControlKind::Jump).pressed() {
	// jumping code
}
```

## Overview

- An [`InputSource`] is a single source of data from a physical input device.
An [`InputSource`] outputs a float from `0.0` to `1.0`. Keys and buttons output
either `0.0` or `1.0`, and gamepad triggers output a float in the range of `0.0`
to `1.0`. Each side of an analog stick axis is considered a separate input
source (e.g. [`LeftStickLeft`](GamepadInput::LeftStickLeft) vs.
[`LeftStickRight`](GamepadInput::LeftStickRight)).
- A [`Control`] is a game-specific virtual input that accumulates the values of
one or more physical input sources. Any control can be used either as an analog
value or a button. A control is considered "held" is its analog value is greater
than the deadzone.
- A [`Stick`] is a game-specific virtual analog stick that combines the values
from 4 controls (left, right, up, and down) into a 2-dimensional vector. This
vector is always clamped to a length of 1. The deadzone can be either circular
or square.
- A [`PlayerInput`] tracks one human's worth of input. It contains a mapping
of [`Control`]s to [`InputSource`], and it tracks state for each virtual
[`Control`] and [`Stick`]. You can get references to [`Control`]s and [`Stick`]s
from the player input by control kind and stick kind, which are user defined
types that implement the [`ControlKind`] and [`StickKind`] traits, respectively.

## Serialization/deserialization

Many types implement `serde::Serialize` and `serde::Deserialize` when the
`serde` feature is enabled.

*/

#![warn(missing_docs)]

pub mod input_source;
pub mod traits;

pub use input_source::*;
pub use traits::{ControlKind, StickKind};

#[cfg(feature = "derive")]
pub use baton_derive::{ControlKind, StickKind};

use std::collections::HashMap;

use input_source::{InputKind, InputSource};
use traits::InputProvider;

/** The shape of a deadzone for a `Stick`. */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DeadzoneShape {
	/** The value of the stick will be non-zero if the magnitude of the
	raw value vector is greater than the deadzone amount. */
	Circle,
	/** The value of each axis of the stick will be non-zero if its
	absolute value is greater than the deadzone amount. */
	Square,
}

/** A configuration for a [`PlayerInput`]. */
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InputConfig<C: ControlKind> {
	/** What input sources should be used for each control kind. */
	pub control_mapping: HashMap<C, Vec<InputSource>>,
	/** The deadzone amount for [`Control`]s and [`Stick`]s. */
	pub deadzone: f32,
	/** The deadzone shape for [`Stick`]s. */
	pub deadzone_shape: DeadzoneShape,
}

/** A game-specific virtual input. */
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Control {
	raw_value: f32,
	value: f32,
	previous_value: f32,
}

impl Control {
	fn new() -> Self {
		Self {
			raw_value: 0.0,
			value: 0.0,
			previous_value: 0.0,
		}
	}

	/** Returns the raw analog value of the control, ignoring deadzone. */
	pub fn raw_value(&self) -> f32 {
		self.raw_value
	}

	/** Returns the analog value of the control with deadzone applied. */
	pub fn value(&self) -> f32 {
		self.value
	}

	/** Returns whether the control (when considered as a button)
	is "held down" this frame. */
	pub fn down(&self) -> bool {
		self.value > 0.0
	}

	/** Returns whether the control (when considered as a button)
	was just pressed this frame. */
	pub fn pressed(&self) -> bool {
		self.value > 0.0 && self.previous_value == 0.0
	}

	/** Returns whether the control (when considered as a button)
	was just released this frame. */
	pub fn released(&self) -> bool {
		self.value == 0.0 && self.previous_value > 0.0
	}
}

/** A game-specific virtual analog stick. */
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Stick {
	raw_value: (f32, f32),
	value: (f32, f32),
}

impl Stick {
	fn new() -> Self {
		Self {
			raw_value: (0.0, 0.0),
			value: (0.0, 0.0),
		}
	}

	/** Returns the raw value of the stick, ignoring deadzone,
	as an 2-dimensional vector. */
	pub fn raw_value(&self) -> (f32, f32) {
		self.raw_value
	}

	/** Returns the value of the stick with deadzone applied
	as an 2-dimensional vector. */
	pub fn value(&self) -> (f32, f32) {
		self.value
	}
}

/** Tracks virtual inputs for one player. */
#[derive(Debug, Clone, PartialEq)]
pub struct PlayerInput<C: ControlKind, P: StickKind<C>, GamepadId> {
	config: InputConfig<C>,
	gamepad: Option<GamepadId>,
	controls: HashMap<C, Control>,
	sticks: HashMap<P, Stick>,
	active_input_kind: InputKind,
}

impl<C: ControlKind, P: StickKind<C>, GamepadId> PlayerInput<C, P, GamepadId> {
	/** Creates a new `PlayerInput` with the given settings. */
	pub fn new(config: InputConfig<C>) -> Self {
		Self {
			config,
			gamepad: None,
			controls: C::all()
				.iter()
				.map(|kind| (*kind, Control::new()))
				.collect(),
			sticks: P::all().iter().map(|kind| (*kind, Stick::new())).collect(),
			active_input_kind: InputKind::Keyboard,
		}
	}

	/** Gets the most recently used [`InputKind`] for this player. */
	pub fn active_input_kind(&self) -> InputKind {
		self.active_input_kind
	}

	/** Assigns (or unsets) a gamepad for this player. */
	pub fn set_gamepad(&mut self, gamepad: impl Into<Option<GamepadId>>) {
		self.gamepad = gamepad.into();
	}

	/** Updates the input state with data from the backend (such as a
	game framework). */
	pub fn update(&mut self, input_provider: impl InputProvider<GamepadId>) {
		self.update_active_input_kind(&input_provider);
		self.update_controls(&input_provider);
		self.update_sticks();
	}

	/** Returns a reference to the [`Control`] of the given kind. */
	pub fn control(&self, kind: C) -> &Control {
		self.controls.get(&kind).unwrap()
	}

	/** Returns a reference to the [`Stick`] of the given kind. */
	pub fn stick(&self, kind: P) -> &Stick {
		self.sticks.get(&kind).unwrap()
	}

	fn update_active_input_kind(&mut self, input_provider: &impl InputProvider<GamepadId>) {
		let mut gamepad_used = false;
		for (_, sources) in &self.config.control_mapping {
			for source in sources {
				if input_provider.raw_value(*source, self.gamepad.as_ref()) > self.config.deadzone {
					match source.kind() {
						InputKind::Keyboard => {
							self.active_input_kind = InputKind::Keyboard;
							return;
						}
						InputKind::Gamepad => {
							gamepad_used = true;
						}
					}
				}
			}
		}
		if gamepad_used {
			self.active_input_kind = InputKind::Gamepad;
		}
	}

	fn update_controls(&mut self, input_provider: &impl InputProvider<GamepadId>) {
		let gamepad = self.gamepad.as_ref();
		let active_input_kind = self.active_input_kind;
		for (kind, control) in &mut self.controls {
			let raw_value = if let Some(sources) = self.config.control_mapping.get(kind) {
				sources
					.iter()
					.filter(|source| source.kind() == active_input_kind)
					.fold(0.0, |previous, source| {
						previous + input_provider.raw_value(*source, gamepad)
					})
					.min(1.0)
			} else {
				0.0
			};
			control.previous_value = control.value;
			control.raw_value = raw_value;
			control.value = if raw_value >= self.config.deadzone {
				raw_value
			} else {
				0.0
			};
		}
	}

	fn update_sticks(&mut self) {
		for (stick_kind, stick) in &mut self.sticks {
			let (left_control_kind, right_control_kind, up_control_kind, down_control_kind) =
				stick_kind.controls();
			let mut raw_x = self.controls.get(&right_control_kind).unwrap().raw_value()
				- self.controls.get(&left_control_kind).unwrap().raw_value();
			let mut raw_y = self.controls.get(&down_control_kind).unwrap().raw_value()
				- self.controls.get(&up_control_kind).unwrap().raw_value();
			let magnitude = (raw_x * raw_x + raw_y * raw_y).sqrt();
			if magnitude > 1.0 {
				raw_x /= magnitude;
				raw_y /= magnitude;
			}
			let (x, y) = match self.config.deadzone_shape {
				DeadzoneShape::Circle => {
					if magnitude >= self.config.deadzone {
						(raw_x, raw_y)
					} else {
						(0.0, 0.0)
					}
				}
				DeadzoneShape::Square => {
					let x = if raw_x.abs() >= self.config.deadzone {
						raw_x
					} else {
						0.0
					};
					let y = if raw_y.abs() >= self.config.deadzone {
						raw_y
					} else {
						0.0
					};
					(x, y)
				}
			};
			stick.raw_value = (raw_x, raw_y);
			stick.value = (x, y);
		}
	}
}
