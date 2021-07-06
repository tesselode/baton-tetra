pub mod input_source;
pub mod traits;

use std::collections::HashMap;

use input_source::InputSource;
use traits::{ControlKind, InputProvider, PairKind};

pub struct InputConfig<C: ControlKind> {
	pub control_mapping: HashMap<C, Vec<InputSource>>,
}

pub struct Control {
	raw_value: f32,
	previous_raw_value: f32,
	value: f32,
	previous_value: f32,
}

impl Control {
	fn new() -> Self {
		Self {
			raw_value: 0.0,
			previous_raw_value: 0.0,
			value: 0.0,
			previous_value: 0.0,
		}
	}

	fn update(&mut self, raw_value: f32, deadzone: f32) {
		self.previous_raw_value = self.raw_value;
		self.previous_value = self.value;
		self.raw_value = raw_value;
		self.value = if raw_value >= deadzone {
			raw_value
		} else {
			0.0
		};
	}

	pub fn raw_value(&self) -> f32 {
		self.raw_value
	}

	pub fn value(&self) -> f32 {
		self.value
	}
}

pub struct Pair;

impl Pair {
	pub fn new() -> Self {
		Self
	}
}

pub struct PlayerInput<C: ControlKind, P: PairKind<C>, GamepadId> {
	config: InputConfig<C>,
	gamepad: Option<GamepadId>,
	controls: HashMap<C, Control>,
	pairs: HashMap<P, Pair>,
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
		}
	}

	pub fn set_gamepad(&mut self, gamepad: impl Into<Option<GamepadId>>) {
		self.gamepad = gamepad.into();
	}

	pub fn update(&mut self, input_provider: impl InputProvider<GamepadId>) {
		let gamepad = self.gamepad.as_ref();
		for (kind, control) in &mut self.controls {
			let raw_value = if let Some(sources) = self.config.control_mapping.get(kind) {
				sources
					.iter()
					.fold(0.0, |previous, source| {
						previous + input_provider.raw_value(*source, gamepad)
					})
					.min(1.0)
			} else {
				0.0
			};
			control.update(raw_value, 0.5);
		}
	}

	pub fn control(&self, kind: C) -> &Control {
		self.controls.get(&kind).unwrap()
	}

	pub fn pair(&self, kind: P) -> &Pair {
		self.pairs.get(&kind).unwrap()
	}
}
