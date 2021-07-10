/*! Traits used by Baton. */

use std::hash::Hash;

use crate::input_source::InputSource;

/** A trait that must be implemented for a type to be used as
a control kind. */
pub trait ControlKind: Sized + Copy + Eq + Hash {
	/** Returns all of the distinct control kinds. */
	fn all<'a>() -> &'a [Self];
}

/** A trait that must be implemented for a type to be used as
a stick kind. */
pub trait StickKind<ControlKind>: Sized + Copy + Eq + Hash {
	/** Returns all of the distinct stick kinds. */
	fn all<'a>() -> &'a [Self];

	/** Returns the controls that this stick kind should encapsulate
	(in the order left, right, up, and down). */
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

/** Converts data from a backend (such as a game framework) to state that
Baton can use. */
pub trait InputProvider<GamepadId> {
	/** Gets the current raw analog value of an input souce with the given gamepad. */
	fn raw_value(&self, source: InputSource, gamepad: Option<&GamepadId>) -> f32;
}
