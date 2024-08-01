use std::fs;

use sdl2::keyboard::Keycode;
use toml::{
    Table,
    Value::{self, Integer},
};

#[derive(Clone, Copy)]
pub struct KeyConfig {
    left: Keycode,
    right: Keycode,
    sdrop: Keycode,
    hdrop: Keycode,
    hold: Keycode,
    rrot: Keycode,
    lrot: Keycode,
}

impl KeyConfig {
    pub fn left(&self) -> Keycode {
        self.left
    }

    pub fn right(&self) -> Keycode {
        self.right
    }

    pub fn sdrop(&self) -> Keycode {
        self.sdrop
    }

    pub fn hdrop(&self) -> Keycode {
        self.hdrop
    }

    pub fn hold(&self) -> Keycode {
        self.hold
    }

    pub fn rrot(&self) -> Keycode {
        self.rrot
    }

    pub fn lrot(&self) -> Keycode {
        self.lrot
    }
}

#[derive(Clone, Copy)]
pub struct Config {
    dir_delay: i64,
    keys: KeyConfig,
}

impl Config {
    pub fn dir_delay(&self) -> i64 {
        self.dir_delay
    }

    pub fn keys(&self) -> &KeyConfig {
        &self.keys
    }
}

pub fn get_config() -> Config {
    match fs::read_to_string("~/.config/config.toml") {
        Ok(c) => parse_map(c.parse().expect("Config is invalid toml")),
        Err(_) => parse_map(Table::new()),
    }
}

fn parse_map(tab: Table) -> Config {
    let dir_delay = match tab.get("dir_delay") {
        Some(Integer(d)) => *d,
        None => 150,
        _ => panic!("Malformed dir_delay value"),
    };

    let left = match tab.get("left") {
        Some(Value::String(s)) => Keycode::from_name(s).expect("Unable to find keycode for left"),
        None => Keycode::Left,
        _ => panic!("Malformed left value"),
    };

    let right = match tab.get("right") {
        Some(Value::String(s)) => Keycode::from_name(s).expect("Unable to find keycode for right"),
        None => Keycode::Right,
        _ => panic!("Malformed right value"),
    };

    let sdrop = match tab.get("sdrop") {
        Some(Value::String(s)) => Keycode::from_name(s).expect("Unable to find keycode for sdrop"),
        None => Keycode::Down,
        _ => panic!("Malformed sdrop value"),
    };

    let hdrop = match tab.get("hdrop") {
        Some(Value::String(s)) => Keycode::from_name(s).expect("Unable to find keycode for hdrop"),
        None => Keycode::Up,
        _ => panic!("Malformed hdrop value"),
    };

    let rrot = match tab.get("rrot") {
        Some(Value::String(s)) => Keycode::from_name(s).expect("Unable to find keycode for rrot"),
        None => Keycode::X,
        _ => panic!("Malformed rrot value"),
    };

    let lrot = match tab.get("lrot") {
        Some(Value::String(s)) => Keycode::from_name(s).expect("Unable to find keycode for lrot"),
        None => Keycode::Z,
        _ => panic!("Malformed lrot value"),
    };

    let hold = match tab.get("hold") {
        Some(Value::String(s)) => Keycode::from_name(s).expect("Unable to find keycode for hold"),
        None => Keycode::LShift,
        _ => panic!("Malformed hold value"),
    };

    Config {
        dir_delay,
        keys: KeyConfig {
            left,
            right,
            sdrop,
            hdrop,
            lrot,
            rrot,
            hold,
        },
    }
}
