use enum_map::Enum;
use tetra::math::Vec2;

use crate::control::Control;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeadzoneShape {
	Square,
	Circle,
}

pub trait PairKindTrait<ControlKind>: Enum<Pair> {
	fn controls(&self) -> (ControlKind, ControlKind, ControlKind, ControlKind);
}

#[derive(Enum)]
pub enum DefaultPairKind {}

impl<ControlKind> PairKindTrait<ControlKind> for DefaultPairKind {
	fn controls(&self) -> (ControlKind, ControlKind, ControlKind, ControlKind) {
		unreachable!()
	}
}

pub struct Pair {
	raw_value: Vec2<f32>,
	value: Vec2<f32>,
}

impl Pair {
	pub(crate) fn new() -> Self {
		Self {
			raw_value: Vec2::zero(),
			value: Vec2::zero(),
		}
	}

	pub(crate) fn update(
		&mut self,
		left: &Control,
		right: &Control,
		up: &Control,
		down: &Control,
		deadzone: f32,
		deadzone_shape: DeadzoneShape,
	) {
		self.raw_value.x = right.raw_value() - left.raw_value();
		self.raw_value.y = down.raw_value() - up.raw_value();
		if self.raw_value.magnitude_squared() > 1.0 {
			self.raw_value.normalize();
		}
		match deadzone_shape {
			DeadzoneShape::Square => {
				self.value.x = if self.raw_value.x.abs() >= deadzone {
					self.raw_value.x
				} else {
					0.0
				};
				self.value.y = if self.raw_value.y.abs() >= deadzone {
					self.raw_value.y
				} else {
					0.0
				};
			}
			DeadzoneShape::Circle => {
				self.value = if self.raw_value.magnitude_squared() >= deadzone.powi(2) {
					self.raw_value
				} else {
					Vec2::zero()
				};
			}
		}
	}

	pub fn raw_value(&self) -> Vec2<f32> {
		self.raw_value
	}

	pub fn value(&self) -> Vec2<f32> {
		self.value
	}
}

impl Default for Pair {
	fn default() -> Self {
		Self::new()
	}
}
