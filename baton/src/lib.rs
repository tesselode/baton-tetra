pub mod input_source;
pub mod traits;

pub use input_source::*;
pub use traits::*;

pub use baton_derive::{ControlKind, PairKind};

use std::collections::HashMap;

use input_source::{InputKind, InputSource};
use traits::{ControlKind, InputProvider, PairKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DeadzoneShape {
	Circle,
	Square,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InputConfig<C: ControlKind> {
	pub control_mapping: HashMap<C, Vec<InputSource>>,
	pub deadzone: f32,
	pub deadzone_shape: DeadzoneShape,
}

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

	pub fn raw_value(&self) -> f32 {
		self.raw_value
	}

	pub fn value(&self) -> f32 {
		self.value
	}

	pub fn down(&self) -> bool {
		self.value > 0.0
	}

	pub fn pressed(&self) -> bool {
		self.value > 0.0 && self.previous_value == 0.0
	}

	pub fn released(&self) -> bool {
		self.value == 0.0 && self.previous_value > 0.0
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pair {
	raw_value: (f32, f32),
	value: (f32, f32),
}

impl Pair {
	fn new() -> Self {
		Self {
			raw_value: (0.0, 0.0),
			value: (0.0, 0.0),
		}
	}

	pub fn raw_value(&self) -> (f32, f32) {
		self.raw_value
	}

	pub fn value(&self) -> (f32, f32) {
		self.value
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlayerInput<C: ControlKind, P: PairKind<C>, GamepadId> {
	config: InputConfig<C>,
	gamepad: Option<GamepadId>,
	controls: HashMap<C, Control>,
	pairs: HashMap<P, Pair>,
	active_input_kind: InputKind,
}

impl<C: ControlKind, P: PairKind<C>, GamepadId> PlayerInput<C, P, GamepadId> {
	pub fn new(config: InputConfig<C>) -> Self {
		Self {
			config,
			gamepad: None,
			controls: C::all()
				.iter()
				.map(|kind| (*kind, Control::new()))
				.collect(),
			pairs: P::all().iter().map(|kind| (*kind, Pair::new())).collect(),
			active_input_kind: InputKind::Keyboard,
		}
	}

	pub fn active_input_kind(&self) -> InputKind {
		self.active_input_kind
	}

	pub fn set_gamepad(&mut self, gamepad: impl Into<Option<GamepadId>>) {
		self.gamepad = gamepad.into();
	}

	pub fn update(&mut self, input_provider: impl InputProvider<GamepadId>) {
		self.update_active_input_kind(&input_provider);
		self.update_controls(&input_provider);
		self.update_pairs();
	}

	pub fn control(&self, kind: C) -> &Control {
		self.controls.get(&kind).unwrap()
	}

	pub fn pair(&self, kind: P) -> &Pair {
		self.pairs.get(&kind).unwrap()
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

	fn update_pairs(&mut self) {
		for (pair_kind, pair) in &mut self.pairs {
			let (left_control_kind, right_control_kind, up_control_kind, down_control_kind) =
				pair_kind.controls();
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
			pair.raw_value = (raw_x, raw_y);
			pair.value = (x, y);
		}
	}
}
