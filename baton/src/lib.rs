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

	pub fn update(&mut self, input_provider: impl InputProvider<GamepadId>) {}
}
