use std::collections::HashMap;

use tetra::Context;

use crate::{
	control::{Control, ControlKindTrait},
	pair::{DeadzoneShape, DefaultPairKind, Pair, PairKindTrait},
	source::{InputKind, InputSource},
};

pub trait ControlConfigTrait<ControlKind: ControlKindTrait> {
	fn control_sources(&self, kind: &ControlKind) -> &[InputSource];

	fn gamepad_id(&self) -> Option<usize> {
		None
	}

	fn deadzone(&self) -> f32 {
		0.5
	}

	fn deadzone_shape(&self) -> DeadzoneShape {
		DeadzoneShape::Circle
	}
}

/// Collects input data for a single player.
pub struct PlayerInput<
	ControlKind: ControlKindTrait,
	ControlConfig: ControlConfigTrait<ControlKind>,
	PairKind: PairKindTrait<ControlKind> = DefaultPairKind,
> {
	/// Settings for the [`PlayerInput`].
	pub config: ControlConfig,
	controls: HashMap<ControlKind, Control>,
	pairs: HashMap<PairKind, Pair>,
	active_input_kind: Option<InputKind>,
}

impl<ControlKind, ControlConfig, PairKind> PlayerInput<ControlKind, ControlConfig, PairKind>
where
	ControlKind: ControlKindTrait,
	ControlConfig: ControlConfigTrait<ControlKind>,
	PairKind: PairKindTrait<ControlKind>,
{
	/// Creates a new [`PlayerInput`].
	pub fn new(config: ControlConfig) -> Self {
		Self {
			config,
			controls: Default::default(),
			pairs: Default::default(),
			active_input_kind: None,
		}
	}

	/// Returns the most recently used input kind, or `None`
	/// if no input devices have been used yet.
	///
	/// Knowing what device the player used last is useful
	/// for some situations, such as showing different
	/// instructions about controls depending on what
	/// device they're using.
	pub fn active_input_kind(&self) -> Option<InputKind> {
		self.active_input_kind
	}

	/// Returns a reference to the control of the specified kind.
	pub fn control(&self, kind: ControlKind) -> &Control {
		self.controls.get(&kind).unwrap()
	}

	/// Returns a reference to the pair of the specified kind.
	pub fn pair(&self, kind: PairKind) -> &Pair {
		self.pairs.get(&kind).unwrap()
	}

	/// Updates the active input kind (keyboard or gamepad).
	///
	/// Only sources of the active kind will be used for
	/// calculatng control values.
	///
	/// If any keyboard key is pressed, the active device
	/// will be set to keyboard. Otherwise, if any gamepad
	/// button is pressed or axis is moved (past the deadzone),
	/// the active device will be set to gamepad. Otherwise,
	/// the active device will remain unchanged.
	fn update_active_input_kind(&mut self, ctx: &Context) {
		let mut gamepad_active = false;
		for kind in ControlKind::kinds() {
			for source in self.config.control_sources(kind) {
				if source.get(ctx, self.config.gamepad_id()) >= self.config.deadzone() {
					if source.kind() == InputKind::Keyboard {
						self.active_input_kind = Some(InputKind::Keyboard);
						return;
					} else if source.kind() == InputKind::Gamepad {
						gamepad_active = true;
					}
				}
			}
		}
		if gamepad_active {
			self.active_input_kind = Some(InputKind::Gamepad);
		}
	}

	/// Updates the [`PlayerInput`] with the input data for this frame.
	pub fn update(&mut self, ctx: &Context) {
		self.update_active_input_kind(ctx);
		for (kind, control) in &mut self.controls {
			control.update(
				ctx,
				self.config.gamepad_id(),
				self.config.control_sources(kind),
				self.config.deadzone(),
				self.active_input_kind,
			);
		}
		for (kind, pair) in &mut self.pairs {
			let (left_control_kind, right_control_kind, up_control_kind, down_control_kind) =
				kind.controls();
			pair.update(
				&self.controls.get(&left_control_kind).unwrap(),
				&self.controls.get(&right_control_kind).unwrap(),
				&self.controls.get(&up_control_kind).unwrap(),
				&self.controls.get(&down_control_kind).unwrap(),
				self.config.deadzone(),
				self.config.deadzone_shape(),
			);
		}
	}
}
