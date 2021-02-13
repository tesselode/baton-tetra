use std::error::Error;

use baton_tetra::{
	player_input::{ControlConfigTrait, PlayerInput},
	source::InputSource,
};
use baton_tetra_derive::{ControlKind, PairKind};
use tetra::{input::Key, Context, ContextBuilder, State};

#[derive(Clone, Copy, PartialEq, Eq, Hash, ControlKind)]
enum ControlKind {
	Left,
	Right,
	Up,
	Down,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PairKind)]
#[control_kind(ControlKind)]
enum PairKind {
	#[controls(Left, Right, Up, Down)]
	Move,
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
