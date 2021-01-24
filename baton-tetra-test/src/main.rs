use tetra::{Context, ContextBuilder, State, TetraError};

struct MainState {}

impl MainState {
	pub fn new() -> Self {
		Self {}
	}
}

impl State for MainState {
	fn update(&mut self, _ctx: &mut Context) -> Result<(), TetraError> {
		Ok(())
	}
}

fn main() -> tetra::Result {
	ContextBuilder::new("baton-tetra-test", 800, 600)
		.build()?
		.run(|_| Ok(MainState::new()))
}
