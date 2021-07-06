#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputKind {
	Keyboard,
	Gamepad,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
	A,
	B,
	C,
	D,
	E,
	F,
	G,
	H,
	I,
	J,
	K,
	L,
	M,
	N,
	O,
	P,
	Q,
	R,
	S,
	T,
	U,
	V,
	W,
	X,
	Y,
	Z,

	Num0,
	Num1,
	Num2,
	Num3,
	Num4,
	Num5,
	Num6,
	Num7,
	Num8,
	Num9,

	F1,
	F2,
	F3,
	F4,
	F5,
	F6,
	F7,
	F8,
	F9,
	F10,
	F11,
	F12,
	F13,
	F14,
	F15,
	F16,
	F17,
	F18,
	F19,
	F20,
	F21,
	F22,
	F23,
	F24,

	NumLock,
	NumPad1,
	NumPad2,
	NumPad3,
	NumPad4,
	NumPad5,
	NumPad6,
	NumPad7,
	NumPad8,
	NumPad9,
	NumPad0,
	NumPadPlus,
	NumPadMinus,
	NumPadMultiply,
	NumPadDivide,
	NumPadEnter,

	LeftCtrl,
	LeftShift,
	LeftAlt,
	RightCtrl,
	RightShift,
	RightAlt,

	Up,
	Down,
	Left,
	Right,

	Ampersand,
	Asterisk,
	At,
	Backquote,
	Backslash,
	Backspace,
	CapsLock,
	Caret,
	Colon,
	Comma,
	Delete,
	Dollar,
	DoubleQuote,
	End,
	Enter,
	Equals,
	Escape,
	Exclaim,
	GreaterThan,
	Hash,
	Home,
	Insert,
	LeftBracket,
	LeftParen,
	LessThan,
	Minus,
	PageDown,
	PageUp,
	Pause,
	Percent,
	Period,
	Plus,
	PrintScreen,
	Question,
	Quote,
	RightBracket,
	RightParen,
	ScrollLock,
	Semicolon,
	Slash,
	Space,
	Tab,
	Underscore,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GamepadInput {
	LeftStickLeft,
	LeftStickRight,
	LeftStickUp,
	LeftStickDown,
	RightStickLeft,
	RightStickRight,
	RightStickUp,
	RightStickDown,
	DpadLeft,
	DpadRight,
	DpadUp,
	DpadDown,
	A,
	B,
	X,
	Y,
	LeftShoulder,
	LeftTrigger,
	LeftStick,
	RightShoulder,
	RightTrigger,
	RightStick,
	Start,
	Back,
	Guide,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputSource {
	Key(Key),
	GamepadInput(GamepadInput),
}

impl InputSource {
	pub fn kind(&self) -> InputKind {
		match self {
			Self::Key(_) => InputKind::Keyboard,
			Self::GamepadInput(_) => InputKind::Gamepad,
		}
	}
}

impl From<Key> for InputSource {
	fn from(key: Key) -> Self {
		Self::Key(key)
	}
}

impl From<GamepadInput> for InputSource {
	fn from(gamepad_input: GamepadInput) -> Self {
		Self::GamepadInput(gamepad_input)
	}
}
