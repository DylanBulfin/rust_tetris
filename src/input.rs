// My goal is to make keybindings fully customizable bc I hate
// programs that don't.

use std::{
    collections::VecDeque,
    time::{Duration, SystemTime},
};

use crate::field::Field;

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

impl KeyEvent {
    pub fn new(key: Key, press: bool) -> Self {
        Self { key, press }
    }
}

pub struct KeyState {
    left: bool,
    right: bool,

    left_press: Option<SystemTime>,
    right_press: Option<SystemTime>,
}

impl KeyState {
    pub fn new() -> Self {
        KeyState {
            left: false,
            right: false,
            left_press: None,
            right_press: None,
        }
    }

    pub fn handle_dirs(&mut self, field: &mut Field) {
        if let Some(st) = self.left_press {
            if self.left
                && SystemTime::now()
                    .duration_since(st)
                    .expect("Something messed up")
                    > Duration::from_millis(100)
            {
                field.snap_left();
            }
        }
        if let Some(st) = self.right_press {
            if self.right
                && SystemTime::now()
                    .duration_since(st)
                    .expect("Something messed up")
                    > Duration::from_millis(100)
            {
                field.snap_right();
            }
        }
    }

    pub fn update(&mut self, event: KeyEvent, field: &mut Field) {
        match event.key {
            Key::Left => {
                self.left = event.press;
                field.piece_left();
                self.left_press = if event.press {
                    Some(SystemTime::now())
                } else {
                    None
                }
            }
            Key::Right => {
                self.right = event.press;
                field.piece_right();
                self.right_press = if event.press {
                    Some(SystemTime::now())
                } else {
                    None
                }
            }
            Key::HDrop => field.drop(true),
            Key::SDrop => field.drop(false),
            Key::RRot => field.rotate_right(),
            Key::LRot => field.rotate_left(),
            Key::Hold => {
                field.next_piece();
            }
        }
    }
}
