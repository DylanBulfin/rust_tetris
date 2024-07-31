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
                    > Duration::from_millis(150)
            {
                field.snap_left();
            }
        }
        if let Some(st) = self.right_press {
            if self.right
                && SystemTime::now()
                    .duration_since(st)
                    .expect("Something messed up")
                    > Duration::from_millis(150)
            {
                field.snap_right();
            }
        }
    }

    pub fn update(&mut self, event: KeyEvent, field: &mut Field) {
        match event.key {
            Key::Left => {
                self.left = event.press;
                if event.press {
                    self.left_press = Some(SystemTime::now());
                    field.piece_left();
                } else {
                    self.left_press = None;
                }
            }
            Key::Right => {
                self.right = event.press;
                if event.press {
                    self.right_press = Some(SystemTime::now());
                    field.piece_right();
                } else {
                    self.right_press = None;
                }
            }
            Key::HDrop => {
                if event.press {
                    field.drop(true)
                } else {
                    ()
                }
            }
            Key::SDrop => {
                if event.press {
                    field.drop(false)
                } else {
                    ()
                }
            }
            Key::RRot => {
                if event.press {
                    field.rotate_right()
                } else {
                    ()
                }
            }
            Key::LRot => {
                if event.press {
                    field.rotate_left()
                } else {
                    ()
                }
            }
            Key::Hold => {
                if event.press {
                    field.hold();
                } else {
                    ()
                };
            }
        }
    }
}
