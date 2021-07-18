use baton::input_source::{GamepadInput, InputSource};
use tetra::{input::GamepadButton, Context};

fn baton_key_to_tetra_key(baton_key: baton::input_source::Key) -> tetra::input::Key {
	match baton_key {
		baton::input_source::Key::A => tetra::input::Key::A,
		baton::input_source::Key::B => tetra::input::Key::B,
		baton::input_source::Key::C => tetra::input::Key::C,
		baton::input_source::Key::D => tetra::input::Key::D,
		baton::input_source::Key::E => tetra::input::Key::E,
		baton::input_source::Key::F => tetra::input::Key::F,
		baton::input_source::Key::G => tetra::input::Key::G,
		baton::input_source::Key::H => tetra::input::Key::H,
		baton::input_source::Key::I => tetra::input::Key::I,
		baton::input_source::Key::J => tetra::input::Key::J,
		baton::input_source::Key::K => tetra::input::Key::K,
		baton::input_source::Key::L => tetra::input::Key::L,
		baton::input_source::Key::M => tetra::input::Key::M,
		baton::input_source::Key::N => tetra::input::Key::N,
		baton::input_source::Key::O => tetra::input::Key::O,
		baton::input_source::Key::P => tetra::input::Key::P,
		baton::input_source::Key::Q => tetra::input::Key::Q,
		baton::input_source::Key::R => tetra::input::Key::R,
		baton::input_source::Key::S => tetra::input::Key::S,
		baton::input_source::Key::T => tetra::input::Key::T,
		baton::input_source::Key::U => tetra::input::Key::U,
		baton::input_source::Key::V => tetra::input::Key::V,
		baton::input_source::Key::W => tetra::input::Key::W,
		baton::input_source::Key::X => tetra::input::Key::X,
		baton::input_source::Key::Y => tetra::input::Key::Y,
		baton::input_source::Key::Z => tetra::input::Key::Z,
		baton::input_source::Key::Num0 => tetra::input::Key::Num0,
		baton::input_source::Key::Num1 => tetra::input::Key::Num1,
		baton::input_source::Key::Num2 => tetra::input::Key::Num2,
		baton::input_source::Key::Num3 => tetra::input::Key::Num3,
		baton::input_source::Key::Num4 => tetra::input::Key::Num4,
		baton::input_source::Key::Num5 => tetra::input::Key::Num5,
		baton::input_source::Key::Num6 => tetra::input::Key::Num6,
		baton::input_source::Key::Num7 => tetra::input::Key::Num7,
		baton::input_source::Key::Num8 => tetra::input::Key::Num8,
		baton::input_source::Key::Num9 => tetra::input::Key::Num9,
		baton::input_source::Key::F1 => tetra::input::Key::F1,
		baton::input_source::Key::F2 => tetra::input::Key::F2,
		baton::input_source::Key::F3 => tetra::input::Key::F3,
		baton::input_source::Key::F4 => tetra::input::Key::F4,
		baton::input_source::Key::F5 => tetra::input::Key::F5,
		baton::input_source::Key::F6 => tetra::input::Key::F6,
		baton::input_source::Key::F7 => tetra::input::Key::F7,
		baton::input_source::Key::F8 => tetra::input::Key::F8,
		baton::input_source::Key::F9 => tetra::input::Key::F9,
		baton::input_source::Key::F10 => tetra::input::Key::F10,
		baton::input_source::Key::F11 => tetra::input::Key::F11,
		baton::input_source::Key::F12 => tetra::input::Key::F12,
		baton::input_source::Key::F13 => tetra::input::Key::F13,
		baton::input_source::Key::F14 => tetra::input::Key::F14,
		baton::input_source::Key::F15 => tetra::input::Key::F15,
		baton::input_source::Key::F16 => tetra::input::Key::F16,
		baton::input_source::Key::F17 => tetra::input::Key::F17,
		baton::input_source::Key::F18 => tetra::input::Key::F18,
		baton::input_source::Key::F19 => tetra::input::Key::F19,
		baton::input_source::Key::F20 => tetra::input::Key::F20,
		baton::input_source::Key::F21 => tetra::input::Key::F21,
		baton::input_source::Key::F22 => tetra::input::Key::F22,
		baton::input_source::Key::F23 => tetra::input::Key::F23,
		baton::input_source::Key::F24 => tetra::input::Key::F24,
		baton::input_source::Key::NumLock => tetra::input::Key::NumLock,
		baton::input_source::Key::NumPad1 => tetra::input::Key::NumPad1,
		baton::input_source::Key::NumPad2 => tetra::input::Key::NumPad2,
		baton::input_source::Key::NumPad3 => tetra::input::Key::NumPad3,
		baton::input_source::Key::NumPad4 => tetra::input::Key::NumPad4,
		baton::input_source::Key::NumPad5 => tetra::input::Key::NumPad5,
		baton::input_source::Key::NumPad6 => tetra::input::Key::NumPad6,
		baton::input_source::Key::NumPad7 => tetra::input::Key::NumPad7,
		baton::input_source::Key::NumPad8 => tetra::input::Key::NumPad8,
		baton::input_source::Key::NumPad9 => tetra::input::Key::NumPad9,
		baton::input_source::Key::NumPad0 => tetra::input::Key::NumPad0,
		baton::input_source::Key::NumPadPlus => tetra::input::Key::NumPadPlus,
		baton::input_source::Key::NumPadMinus => tetra::input::Key::NumPadMinus,
		baton::input_source::Key::NumPadMultiply => tetra::input::Key::NumPadMultiply,
		baton::input_source::Key::NumPadDivide => tetra::input::Key::NumPadDivide,
		baton::input_source::Key::NumPadEnter => tetra::input::Key::NumPadEnter,
		baton::input_source::Key::LeftCtrl => tetra::input::Key::LeftCtrl,
		baton::input_source::Key::LeftShift => tetra::input::Key::LeftShift,
		baton::input_source::Key::LeftAlt => tetra::input::Key::LeftAlt,
		baton::input_source::Key::RightCtrl => tetra::input::Key::RightCtrl,
		baton::input_source::Key::RightShift => tetra::input::Key::RightShift,
		baton::input_source::Key::RightAlt => tetra::input::Key::RightAlt,
		baton::input_source::Key::Up => tetra::input::Key::Up,
		baton::input_source::Key::Down => tetra::input::Key::Down,
		baton::input_source::Key::Left => tetra::input::Key::Left,
		baton::input_source::Key::Right => tetra::input::Key::Right,
		baton::input_source::Key::Ampersand => tetra::input::Key::Ampersand,
		baton::input_source::Key::Asterisk => tetra::input::Key::Asterisk,
		baton::input_source::Key::At => tetra::input::Key::At,
		baton::input_source::Key::Backquote => tetra::input::Key::Backquote,
		baton::input_source::Key::Backslash => tetra::input::Key::Backslash,
		baton::input_source::Key::Backspace => tetra::input::Key::Backspace,
		baton::input_source::Key::CapsLock => tetra::input::Key::CapsLock,
		baton::input_source::Key::Caret => tetra::input::Key::Caret,
		baton::input_source::Key::Colon => tetra::input::Key::Colon,
		baton::input_source::Key::Comma => tetra::input::Key::Comma,
		baton::input_source::Key::Delete => tetra::input::Key::Delete,
		baton::input_source::Key::Dollar => tetra::input::Key::Dollar,
		baton::input_source::Key::DoubleQuote => tetra::input::Key::DoubleQuote,
		baton::input_source::Key::End => tetra::input::Key::End,
		baton::input_source::Key::Enter => tetra::input::Key::Enter,
		baton::input_source::Key::Equals => tetra::input::Key::Equals,
		baton::input_source::Key::Escape => tetra::input::Key::Escape,
		baton::input_source::Key::Exclaim => tetra::input::Key::Exclaim,
		baton::input_source::Key::GreaterThan => tetra::input::Key::GreaterThan,
		baton::input_source::Key::Hash => tetra::input::Key::Hash,
		baton::input_source::Key::Home => tetra::input::Key::Home,
		baton::input_source::Key::Insert => tetra::input::Key::Insert,
		baton::input_source::Key::LeftBracket => tetra::input::Key::LeftBracket,
		baton::input_source::Key::LeftParen => tetra::input::Key::LeftParen,
		baton::input_source::Key::LessThan => tetra::input::Key::LessThan,
		baton::input_source::Key::Minus => tetra::input::Key::Minus,
		baton::input_source::Key::PageDown => tetra::input::Key::PageDown,
		baton::input_source::Key::PageUp => tetra::input::Key::PageUp,
		baton::input_source::Key::Pause => tetra::input::Key::Pause,
		baton::input_source::Key::Percent => tetra::input::Key::Percent,
		baton::input_source::Key::Period => tetra::input::Key::Period,
		baton::input_source::Key::Plus => tetra::input::Key::Plus,
		baton::input_source::Key::PrintScreen => tetra::input::Key::PrintScreen,
		baton::input_source::Key::Question => tetra::input::Key::Question,
		baton::input_source::Key::Quote => tetra::input::Key::Quote,
		baton::input_source::Key::RightBracket => tetra::input::Key::RightBracket,
		baton::input_source::Key::RightParen => tetra::input::Key::RightParen,
		baton::input_source::Key::ScrollLock => tetra::input::Key::ScrollLock,
		baton::input_source::Key::Semicolon => tetra::input::Key::Semicolon,
		baton::input_source::Key::Slash => tetra::input::Key::Slash,
		baton::input_source::Key::Space => tetra::input::Key::Space,
		baton::input_source::Key::Tab => tetra::input::Key::Tab,
		baton::input_source::Key::Underscore => tetra::input::Key::Underscore,
	}
}

fn gamepad_input_to_gamepad_button(gamepad_input: GamepadInput) -> Option<GamepadButton> {
	match gamepad_input {
		GamepadInput::DpadLeft => Some(GamepadButton::Left),
		GamepadInput::DpadRight => Some(GamepadButton::Right),
		GamepadInput::DpadUp => Some(GamepadButton::Up),
		GamepadInput::DpadDown => Some(GamepadButton::Down),
		GamepadInput::A => Some(GamepadButton::A),
		GamepadInput::B => Some(GamepadButton::B),
		GamepadInput::X => Some(GamepadButton::X),
		GamepadInput::Y => Some(GamepadButton::Y),
		GamepadInput::LeftShoulder => Some(GamepadButton::LeftShoulder),
		GamepadInput::LeftStick => Some(GamepadButton::LeftStick),
		GamepadInput::RightShoulder => Some(GamepadButton::RightShoulder),
		GamepadInput::RightStick => Some(GamepadButton::RightStick),
		GamepadInput::Start => Some(GamepadButton::Start),
		GamepadInput::Back => Some(GamepadButton::Back),
		GamepadInput::Guide => Some(GamepadButton::Guide),
		_ => None,
	}
}

pub struct InputProvider<'a>(pub &'a Context);

impl<'a> baton::traits::InputProvider<usize> for InputProvider<'a> {
	fn raw_value(&self, source: InputSource, gamepad: Option<&usize>) -> f32 {
		match source {
			InputSource::Key(key) => {
				if tetra::input::is_key_down(self.0, baton_key_to_tetra_key(key)) {
					1.0
				} else {
					0.0
				}
			}
			InputSource::GamepadInput(input) => {
				if let Some(gamepad) = gamepad {
					match input {
						GamepadInput::LeftStickLeft => (-tetra::input::get_gamepad_axis_position(
							self.0,
							*gamepad,
							tetra::input::GamepadAxis::LeftStickX,
						))
						.max(0.0),
						GamepadInput::LeftStickRight => tetra::input::get_gamepad_axis_position(
							self.0,
							*gamepad,
							tetra::input::GamepadAxis::LeftStickX,
						)
						.max(0.0),
						GamepadInput::LeftStickUp => (-tetra::input::get_gamepad_axis_position(
							self.0,
							*gamepad,
							tetra::input::GamepadAxis::LeftStickY,
						))
						.max(0.0),
						GamepadInput::LeftStickDown => tetra::input::get_gamepad_axis_position(
							self.0,
							*gamepad,
							tetra::input::GamepadAxis::LeftStickY,
						)
						.max(0.0),
						GamepadInput::RightStickLeft => (-tetra::input::get_gamepad_axis_position(
							self.0,
							*gamepad,
							tetra::input::GamepadAxis::RightStickX,
						))
						.max(0.0),
						GamepadInput::RightStickRight => tetra::input::get_gamepad_axis_position(
							self.0,
							*gamepad,
							tetra::input::GamepadAxis::RightStickX,
						)
						.max(0.0),
						GamepadInput::RightStickUp => (-tetra::input::get_gamepad_axis_position(
							self.0,
							*gamepad,
							tetra::input::GamepadAxis::RightStickY,
						))
						.max(0.0),
						GamepadInput::RightStickDown => tetra::input::get_gamepad_axis_position(
							self.0,
							*gamepad,
							tetra::input::GamepadAxis::RightStickY,
						)
						.max(0.0),
						GamepadInput::LeftTrigger => tetra::input::get_gamepad_axis_position(
							self.0,
							*gamepad,
							tetra::input::GamepadAxis::LeftTrigger,
						),
						GamepadInput::RightTrigger => tetra::input::get_gamepad_axis_position(
							self.0,
							*gamepad,
							tetra::input::GamepadAxis::RightTrigger,
						),
						input => {
							if tetra::input::is_gamepad_button_down(
								self.0,
								*gamepad,
								gamepad_input_to_gamepad_button(input).unwrap(),
							) {
								1.0
							} else {
								0.0
							}
						}
					}
				} else {
					0.0
				}
			}
		}
	}
}
