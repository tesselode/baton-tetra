use std::{collections::HashMap, error::Error};

use baton::{DeadzoneShape, GamepadInput, InputConfig, Key, PlayerInput};
use tetra::{Context, ContextBuilder, State};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, baton::ControlKind)]
enum ControlKind {
	Left,
	Right,
	Up,
	Down,
	Jump,
	Run,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, baton::StickKind)]
#[control_kind(ControlKind)]
enum StickKind {
	#[controls(Left, Right, Up, Down)]
	Move,
}

struct MainState {
	input: PlayerInput<ControlKind, StickKind, usize>,
}

impl MainState {
	pub fn new() -> Self {
		Self {
			input: PlayerInput::new(InputConfig {
				control_mapping: {
					let mut control_mapping = HashMap::new();
					control_mapping.insert(
						ControlKind::Left,
						vec![Key::A.into(), GamepadInput::LeftStickLeft.into()],
					);
					control_mapping.insert(
						ControlKind::Right,
						vec![Key::D.into(), GamepadInput::LeftStickRight.into()],
					);
					control_mapping.insert(
						ControlKind::Up,
						vec![Key::W.into(), GamepadInput::LeftStickUp.into()],
					);
					control_mapping.insert(
						ControlKind::Down,
						vec![Key::S.into(), GamepadInput::LeftStickDown.into()],
					);
					control_mapping.insert(
						ControlKind::Jump,
						vec![Key::X.into(), GamepadInput::A.into()],
					);
					control_mapping.insert(
						ControlKind::Run,
						vec![Key::Z.into(), GamepadInput::B.into()],
					);
					control_mapping
				},
				deadzone: 0.25,
				deadzone_shape: DeadzoneShape::Circle,
			}),
		}
	}
}

impl State<Box<dyn Error>> for MainState {
	fn update(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		self.input.update(baton_tetra::InputProvider(ctx));
		let (move_x, move_y) = self.input.stick(StickKind::Move).value();
		if self.input.control(ControlKind::Jump).pressed() {
			// jumping code
		}
		Ok(())
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	ContextBuilder::new("Baton demo", 800, 600)
		.build()?
		.run(|_| Ok(MainState::new()))
}
