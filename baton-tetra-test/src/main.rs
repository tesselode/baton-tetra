use baton_tetra::{
	enum_map::{self, Enum},
	pair::PairDeadzoneShape,
	player_input::PlayerInput,
	source::AxisDirection,
};
use baton_tetra::{pair::PairKindTrait, player_input::ControlConfig};
use enum_map::enum_map;
use tetra::{
	input::{GamepadAxis, Key},
	Context, ContextBuilder, State, TetraError,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Enum)]
enum ControlKind {
	Left,
	Right,
	Up,
	Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Enum)]
enum PairKind {
	Move,
}

impl PairKindTrait<ControlKind> for PairKind {
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
	input: PlayerInput<ControlKind, PairKind>,
}

impl MainState {
	pub fn new() -> Self {
		Self {
			input: PlayerInput::new(ControlConfig {
				control_sources: enum_map! {
					ControlKind::Left => vec![
						Key::Left.into(),
						(GamepadAxis::LeftStickX, AxisDirection::Negative).into(),
					],
					ControlKind::Right => vec![
						Key::Right.into(),
						(GamepadAxis::LeftStickX, AxisDirection::Positive).into(),
					],
					ControlKind::Up => vec![
						Key::Up.into(),
						(GamepadAxis::LeftStickY, AxisDirection::Negative).into(),
					],
					ControlKind::Down => vec![
						Key::Down.into(),
						(GamepadAxis::LeftStickY, AxisDirection::Positive).into(),
					],
				},
				gamepad_id: Some(0),
				deadzone: 0.25,
				deadzone_shape: PairDeadzoneShape::Circle,
			}),
		}
	}
}

impl State for MainState {
	fn update(&mut self, ctx: &mut Context) -> Result<(), TetraError> {
		self.input.update(ctx);
		println!("{}", self.input.pair(PairKind::Move).value());
		Ok(())
	}
}

fn main() -> tetra::Result {
	ContextBuilder::new("baton-tetra-test", 800, 600)
		.build()?
		.run(|_| Ok(MainState::new()))
}
