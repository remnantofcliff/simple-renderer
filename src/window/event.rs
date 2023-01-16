use fermium::prelude::*;

#[derive(Debug)]
pub enum Event {
    KeyPressed { scancode: Key, repeat: bool },
    KeyReleased { scancode: Key },
    Resized(i32, i32),
    Unknown,
    Quit,
}

impl Event {
    pub fn from_sdl_event(event: SDL_Event) -> Event {
        unsafe {
            match event.type_ {
                SDL_KEYDOWN => Event::KeyPressed {
                    scancode: Key::from_sdl_scancode(event.key.keysym.scancode),
                    repeat: event.key.repeat != 0,
                },
                SDL_KEYUP => Event::KeyReleased {
                    scancode: Key::from_sdl_scancode(event.key.keysym.scancode),
                },
                SDL_WINDOWEVENT => match event.window.event {
                    SDL_WINDOWEVENT_RESIZED => {
                        let (w, h) = (event.window.data1, event.window.data2);

                        Event::Resized(w, h)
                    }
                    _ => Event::Unknown,
                },
                SDL_QUIT => Event::Quit,
                _ => Event::Unknown,
            }
        }
    }
}

#[derive(Debug)]
#[repr(i32)]
pub enum Key {
    A = 4,
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
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    _0,
    Return,
    Escape,
    Backspace,
    Tab,
    Space,
    Minus,
    Equals,
    LeftBracket,
    RightBracket,
    Backslash,
    Grave,
    Comma,
    Period,
    Slash,
    Caps,
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
    PrintScreen,
    ScrollLock,
    Pause,
    Insert,
    Home,
    PageUp,
    Delete,
    End,
    PageDown,
    Right,
    Left,
    Down,
    Up,
    NumLock,
    NumpadDivide,
    NumpadMultiply,
    NumpadMinus,
    NumpadPlus,
    NumpadEnter,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    Numpad0,
    NumpadPeriod,
    LeftControl = 224,
    LeftShift,
    LeftAlt,
    LeftSuper,
    RightControl,
    RightShift,
    RightAlt,
    RightSuper,

    Unknown = 512,
}

impl Key {
    pub fn from_sdl_scancode(scancode: SDL_Scancode) -> Self {
        match scancode {
            SDL_SCANCODE_A => Self::A,
            SDL_SCANCODE_B => Self::B,
            SDL_SCANCODE_C => Self::C,
            SDL_SCANCODE_D => Self::D,
            SDL_SCANCODE_E => Self::E,
            SDL_SCANCODE_F => Self::F,
            SDL_SCANCODE_G => Self::G,
            SDL_SCANCODE_H => Self::H,
            SDL_SCANCODE_I => Self::I,
            SDL_SCANCODE_J => Self::J,
            SDL_SCANCODE_K => Self::K,
            SDL_SCANCODE_L => Self::L,
            SDL_SCANCODE_M => Self::M,
            SDL_SCANCODE_N => Self::N,
            SDL_SCANCODE_O => Self::O,
            SDL_SCANCODE_P => Self::P,
            SDL_SCANCODE_Q => Self::Q,
            SDL_SCANCODE_R => Self::R,
            SDL_SCANCODE_S => Self::S,
            SDL_SCANCODE_T => Self::T,
            SDL_SCANCODE_U => Self::U,
            SDL_SCANCODE_V => Self::V,
            SDL_SCANCODE_W => Self::W,
            SDL_SCANCODE_X => Self::X,
            SDL_SCANCODE_Y => Self::Y,
            SDL_SCANCODE_Z => Self::Z,
            SDL_SCANCODE_1 => Self::_1,
            SDL_SCANCODE_2 => Self::_2,
            SDL_SCANCODE_3 => Self::_3,
            SDL_SCANCODE_4 => Self::_4,
            SDL_SCANCODE_5 => Self::_5,
            SDL_SCANCODE_6 => Self::_6,
            SDL_SCANCODE_7 => Self::_7,
            SDL_SCANCODE_8 => Self::_8,
            SDL_SCANCODE_9 => Self::_9,
            SDL_SCANCODE_0 => Self::_0,
            SDL_SCANCODE_RETURN => Self::Return,
            SDL_SCANCODE_ESCAPE => Self::Escape,
            SDL_SCANCODE_BACKSPACE => Self::Backspace,
            SDL_SCANCODE_TAB => Self::Tab,
            SDL_SCANCODE_SPACE => Self::Space,
            SDL_SCANCODE_MINUS => Self::Minus,
            SDL_SCANCODE_EQUALS => Self::Equals,
            SDL_SCANCODE_LEFTBRACKET => Self::LeftBracket,
            SDL_SCANCODE_RIGHTBRACKET => Self::RightBracket,
            SDL_SCANCODE_BACKSLASH => Self::Backslash,
            SDL_SCANCODE_GRAVE => Self::Grave,
            SDL_SCANCODE_COMMA => Self::Comma,
            SDL_SCANCODE_PERIOD => Self::Period,
            SDL_SCANCODE_SLASH => Self::Slash,
            SDL_SCANCODE_CAPSLOCK => Self::Caps,
            SDL_SCANCODE_F1 => Self::F1,
            SDL_SCANCODE_F2 => Self::F2,
            SDL_SCANCODE_F3 => Self::F3,
            SDL_SCANCODE_F4 => Self::F4,
            SDL_SCANCODE_F5 => Self::F5,
            SDL_SCANCODE_F6 => Self::F6,
            SDL_SCANCODE_F7 => Self::F7,
            SDL_SCANCODE_F8 => Self::F8,
            SDL_SCANCODE_F9 => Self::F9,
            SDL_SCANCODE_F10 => Self::F10,
            SDL_SCANCODE_F11 => Self::F11,
            SDL_SCANCODE_F12 => Self::F12,
            SDL_SCANCODE_PRINTSCREEN => Self::PrintScreen,
            SDL_SCANCODE_SCROLLLOCK => Self::ScrollLock,
            SDL_SCANCODE_PAUSE => Self::Pause,
            SDL_SCANCODE_INSERT => Self::Insert,
            SDL_SCANCODE_HOME => Self::Home,
            SDL_SCANCODE_PAGEUP => Self::PageUp,
            SDL_SCANCODE_DELETE => Self::Delete,
            SDL_SCANCODE_END => Self::End,
            SDL_SCANCODE_PAGEDOWN => Self::PageDown,
            SDL_SCANCODE_RIGHT => Self::Right,
            SDL_SCANCODE_LEFT => Self::Left,
            SDL_SCANCODE_DOWN => Self::Down,
            SDL_SCANCODE_UP => Self::Up,
            SDL_SCANCODE_NUMLOCKCLEAR => Self::NumLock,
            SDL_SCANCODE_KP_DIVIDE => Self::NumpadDivide,
            SDL_SCANCODE_KP_MULTIPLY => Self::NumpadMultiply,
            SDL_SCANCODE_KP_MINUS => Self::NumpadMinus,
            SDL_SCANCODE_KP_PLUS => Self::NumpadPlus,
            SDL_SCANCODE_KP_ENTER => Self::NumpadEnter,
            SDL_SCANCODE_KP_1 => Self::Numpad1,
            SDL_SCANCODE_KP_2 => Self::Numpad2,
            SDL_SCANCODE_KP_3 => Self::Numpad3,
            SDL_SCANCODE_KP_4 => Self::Numpad4,
            SDL_SCANCODE_KP_5 => Self::Numpad5,
            SDL_SCANCODE_KP_6 => Self::Numpad6,
            SDL_SCANCODE_KP_7 => Self::Numpad7,
            SDL_SCANCODE_KP_8 => Self::Numpad8,
            SDL_SCANCODE_KP_9 => Self::Numpad9,
            SDL_SCANCODE_KP_0 => Self::Numpad0,
            SDL_SCANCODE_KP_PERIOD => Self::NumpadPeriod,
            SDL_SCANCODE_LCTRL => Self::LeftControl,
            SDL_SCANCODE_LSHIFT => Self::LeftShift,
            SDL_SCANCODE_LALT => Self::LeftAlt,
            SDL_SCANCODE_LGUI => Self::LeftSuper,
            SDL_SCANCODE_RCTRL => Self::RightControl,
            SDL_SCANCODE_RSHIFT => Self::RightShift,
            SDL_SCANCODE_RALT => Self::RightAlt,
            SDL_SCANCODE_RGUI => Self::RightSuper,
            _ => Self::Unknown,
        }
    }
}