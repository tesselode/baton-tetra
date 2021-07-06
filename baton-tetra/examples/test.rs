use std::{collections::HashMap, error::Error};

use baton::{
	input_source::{GamepadInput, Key},
	DeadzoneShape, InputConfig, PlayerInput,
};
use baton_tetra::InputProvider;
use tetra::{ContextBuilder, State};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ControlKind {
	Left,
	Right,
	Up,
	Down,
}

impl baton::traits::ControlKind for ControlKind {
	fn all<'a>() -> &'a [Self] {
		&[Self::Left, Self::Right, Self::Up, Self::Down]
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum PairKind {
	Move,
}

impl baton::traits::PairKind<ControlKind> for PairKind {
	fn all<'a>() -> &'a [Self] {
		&[Self::Move]
	}

	fn controls(&self) -> (ControlKind, ControlKind, ControlKind, ControlKind) {
		match self {
			PairKind::Move => (
				ControlKind::Left,
				ControlKind::Right,
				ControlKind::Up,
				ControlKind::Down,
			),
		}
	}
}

struct MainState {
	player_input: PlayerInput<ControlKind, PairKind, usize>,
}

impl MainState {
	fn new() -> Self {
		Self {
			player_input: {
				let mut player_input = PlayerInput::new(InputConfig {
					control_mapping: {
						let mut control_mapping = HashMap::new();
						control_mapping.insert(
							ControlKind::Left,
							vec![Key::Left.into(), GamepadInput::LeftStickLeft.into()],
						);
						control_mapping.insert(
							ControlKind::Right,
							vec![Key::Right.into(), GamepadInput::LeftStickRight.into()],
						);
						control_mapping.insert(
							ControlKind::Up,
							vec![Key::Up.into(), GamepadInput::LeftStickUp.into()],
						);
						control_mapping.insert(
							ControlKind::Down,
							vec![Key::Down.into(), GamepadInput::LeftStickDown.into()],
						);
						control_mapping
					},
					deadzone: 0.25,
					deadzone_shape: DeadzoneShape::Square,
				});
				player_input.set_gamepad(0);
				player_input
			},
		}
	}
}

impl State<Box<dyn Error>> for MainState {
	fn update(&mut self, ctx: &mut tetra::Context) -> Result<(), Box<dyn Error>> {
		self.player_input.update(InputProvider(ctx));
		let pair = self.player_input.pair(PairKind::Move);
		println!("{:?}", pair.value());
		Ok(())
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	ContextBuilder::new("test", 800, 600)
		.build()?
		.run(|_| Ok(MainState::new()))
}
