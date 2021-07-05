use std::collections::HashMap;

pub trait ControlKind: Sized {
	fn all(&self) -> &[Self];
}

pub trait PairKind<ControlKind>: Sized {
	fn all(&self) -> &[Self];

	fn controls(&self) -> (ControlKind, ControlKind, ControlKind, ControlKind);
}

impl<ControlKind> PairKind<ControlKind> for () {
	fn all(&self) -> &[Self] {
		&[]
	}

	fn controls(&self) -> (ControlKind, ControlKind, ControlKind, ControlKind) {
		unreachable!()
	}
}

enum InputSource {}

pub struct ControlMapping<C: ControlKind>(HashMap<C, Vec<InputSource>>);

pub struct InputConfig<C: ControlKind> {
	pub control_mapping: ControlMapping<C>,
}

pub struct Control;

pub struct Pair;

pub struct PlayerInput<C: ControlKind, P: PairKind<C>> {
	controls: HashMap<C, Control>,
	pairs: HashMap<P, Pair>,
}

impl<C: ControlKind, P: PairKind<C>> PlayerInput<C, P> {
	pub fn new() -> Self {
		Self {
			controls: todo!(),
			pairs: todo!(),
		}
	}
}
