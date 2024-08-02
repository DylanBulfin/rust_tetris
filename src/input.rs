// My goal is to make keybindings fully customizable bc I hate
// programs that don't.

use std::time::{Duration, SystemTime};

use crate::{config::Config, state::State};

pub enum Key {
    Left,
    Right,
    HDrop,
    SDrop,
    RRot,
    LRot,
    Hold, // Later, i hope
}

pub struct KeyEvent {
    pub(crate) key: Key,
    pub(crate) press: bool,
}

pub struct KeyState {
    config: Config,

    left: bool,
    right: bool,

    sdrop: bool,

    left_press: Option<SystemTime>,
    right_press: Option<SystemTime>,
}

impl KeyState {
    pub fn new(config: Config) -> Self {
        KeyState {
            config,
            left: false,
            right: false,
            sdrop: false,
            left_press: None,
            right_press: None,
        }
    }

    pub fn handle_special(&mut self, state: &mut State) {
        if self.sdrop {
            state.drop(false)
        }

        if let Some(st) = self.left_press {
            if self.left
                && SystemTime::now()
                    .duration_since(st)
                    .expect("Something messed up")
                    > Duration::from_millis(self.config.dir_delay() as u64)
            {
                state.snap_left();
            }
        }
        if let Some(st) = self.right_press {
            if self.right
                && SystemTime::now()
                    .duration_since(st)
                    .expect("Something messed up")
                    > Duration::from_millis(self.config.dir_delay() as u64)
            {
                state.snap_right();
            }
        }
    }

    pub fn update(&mut self, event: KeyEvent, state: &mut State) {
        match event.key {
            Key::Left => {
                self.left = event.press;
                if event.press {
                    self.left_press = Some(SystemTime::now());
                    state.piece_left();
                } else {
                    self.left_press = None;
                }
            }
            Key::Right => {
                self.right = event.press;
                if event.press {
                    self.right_press = Some(SystemTime::now());
                    state.piece_right();
                } else {
                    self.right_press = None;
                }
            }
            Key::HDrop => {
                if event.press {
                    state.drop(true)
                }
            }
            Key::SDrop => {
                if event.press {
                    self.sdrop = true;
                    state.drop(false)
                } else {
                    self.sdrop = false;
                }
            }
            Key::RRot => {
                if event.press {
                    state.rotate_right()
                }
            }
            Key::LRot => {
                if event.press {
                    state.rotate_left()
                }
            }
            Key::Hold => {
                if event.press {
                    state.hold();
                }
            }
        }
    }
}
