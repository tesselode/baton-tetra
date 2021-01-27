use enum_map::{Enum, EnumMap};
use tetra::Context;

use crate::{
	control::Control,
	pair::{DeadzoneShape, DefaultPairKind, Pair, PairKindTrait},
	source::{InputKind, InputSource},
};

pub struct ControlConfig<ControlKind: Enum<Vec<InputSource>>> {
	pub control_sources: EnumMap<ControlKind, Vec<InputSource>>,
	pub gamepad_id: Option<usize>,
	pub deadzone: f32,
	pub deadzone_shape: DeadzoneShape,
}

pub struct PlayerInput<
	ControlKind: Enum<Control> + Enum<Vec<InputSource>>,
	PairKind: PairKindTrait<ControlKind> = DefaultPairKind,
> {
	pub config: ControlConfig<ControlKind>,
	controls: EnumMap<ControlKind, Control>,
	pairs: EnumMap<PairKind, Pair>,
	active_input_kind: Option<InputKind>,
}

impl<ControlKind: Enum<Control> + Enum<Vec<InputSource>>, PairKind: PairKindTrait<ControlKind>>
	PlayerInput<ControlKind, PairKind>
{
	pub fn new(config: ControlConfig<ControlKind>) -> Self {
		Self {
			config,
			controls: Default::default(),
			pairs: Default::default(),
			active_input_kind: None,
		}
	}

	pub fn active_input_kind(&self) -> Option<InputKind> {
		self.active_input_kind
	}

	pub fn control(&self, kind: ControlKind) -> &Control {
		&self.controls[kind]
	}

	pub fn pair(&self, kind: PairKind) -> &Pair {
		&self.pairs[kind]
	}

	pub fn set_gamepad(&mut self, id: impl Into<Option<usize>>) {
		self.config.gamepad_id = id.into();
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
		for (_, sources) in &self.config.control_sources {
			for source in sources {
				if source.get(ctx, self.config.gamepad_id) >= self.config.deadzone {
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

	pub fn update(&mut self, ctx: &Context) {
		self.update_active_input_kind(ctx);
		for (kind, control) in &mut self.controls {
			control.update(
				ctx,
				self.config.gamepad_id,
				&self.config.control_sources[kind],
				self.config.deadzone,
				self.active_input_kind,
			);
		}
		for (kind, pair) in &mut self.pairs {
			let (left_control_kind, right_control_kind, up_control_kind, down_control_kind) =
				kind.controls();
			pair.update(
				&self.controls[left_control_kind],
				&self.controls[right_control_kind],
				&self.controls[up_control_kind],
				&self.controls[down_control_kind],
				self.config.deadzone,
				self.config.deadzone_shape,
			);
		}
	}
}
