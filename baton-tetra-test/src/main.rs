use std::error::Error;

use baton_tetra::{
	control::ControlKindTrait,
	pair::PairKindTrait,
	player_input::{ControlConfigTrait, PlayerInput},
	source::InputSource,
};
use tetra::{input::Key, Context, ContextBuilder, State};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum ControlKind {
	Left,
	Right,
	Up,
	Down,
}

impl ControlKindTrait for ControlKind {
	fn kinds() -> &'static [Self] {
		&[Self::Left, Self::Right, Self::Up, Self::Down]
	}
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum PairKind {
	Move,
}

impl PairKindTrait<ControlKind> for PairKind {
	fn kinds() -> &'static [Self] {
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

struct ControlConfig {
	left: Vec<InputSource>,
	right: Vec<InputSource>,
	up: Vec<InputSource>,
	down: Vec<InputSource>,
}

impl ControlConfig {
	pub fn new() -> Self {
		Self {
			left: vec![Key::Left.into()],
			right: vec![Key::Right.into()],
			up: vec![Key::Up.into()],
			down: vec![Key::Down.into()],
		}
	}
}

impl ControlConfigTrait<ControlKind> for ControlConfig {
	fn control_sources(&self, kind: &ControlKind) -> &[InputSource] {
		match kind {
			ControlKind::Left => &self.left,
			ControlKind::Right => &self.right,
			ControlKind::Up => &self.up,
			ControlKind::Down => &self.down,
		}
	}
}

struct MainState {
	player_input: PlayerInput<ControlKind, ControlConfig, PairKind>,
}

impl MainState {
	pub fn new() -> Self {
		Self {
			player_input: PlayerInput::new(ControlConfig::new()),
		}
	}
}

impl State<Box<dyn Error>> for MainState {
	fn update(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		self.player_input.update(ctx);
		println!("{}", self.player_input.pair(PairKind::Move).value());
		Ok(())
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	ContextBuilder::new("baton-tetra-test", 1280, 720)
		.build()?
		.run(|_| Ok(MainState::new()))
}
