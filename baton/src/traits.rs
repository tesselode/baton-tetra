use std::hash::Hash;

use crate::input_source::InputSource;

pub trait ControlKind: Sized + Copy + Eq + Hash {
	fn all<'a>() -> &'a [Self];
}

pub trait StickKind<ControlKind>: Sized + Copy + Eq + Hash {
	fn all<'a>() -> &'a [Self];

	fn controls(&self) -> (ControlKind, ControlKind, ControlKind, ControlKind);
}

impl<ControlKind> StickKind<ControlKind> for () {
	fn all<'a>() -> &'a [Self] {
		&[]
	}

	fn controls(&self) -> (ControlKind, ControlKind, ControlKind, ControlKind) {
		unreachable!()
	}
}

pub trait InputProvider<GamepadId> {
	fn raw_value(&self, source: InputSource, gamepad: Option<&GamepadId>) -> f32;
}
