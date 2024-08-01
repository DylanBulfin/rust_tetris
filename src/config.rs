use std::fs;

use sdl2::keyboard::Keycode;
use toml::{
    Table,
    Value::{self, Integer},
};

use crate::TetrErr;

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

pub fn get_config() -> Result<Config, TetrErr> {
    let mut path = homedir::my_home()?.expect("Unable to find home dir");
    path.push(".config/tetrs/config.toml");
    println!("{}", path.to_str().unwrap());
    match fs::read_to_string(path) {
        Ok(c) => {
            println!("{}", c);
            Ok(parse_map(c.parse().expect("Config is invalid toml")))
        }
        Err(_) => Ok(parse_map(Table::new())),
    }
}

fn parse_map(tab: Table) -> Config {
    println!("{:?}", tab);
    let dir_delay = match tab.get("dir_delay") {
        Some(Integer(d)) => *d,
        None => 150,
        _ => panic!("Malformed dir_delay value"),
    };
    
    let keys = match tab.get("keys") {
        Some(Value::Table(t)) => t,
        None => &Table::new(),
        _ => panic!("Malformed keys"),
    };
    
    let left = match keys.get("left") {
        Some(Value::String(s)) => Keycode::from_name(s).expect("Unable to find keycode for left"),
        None => Keycode::Left,
        _ => panic!("Malformed left value"),
    };

    let right = match keys.get("right") {
        Some(Value::String(s)) => Keycode::from_name(s).expect("Unable to find keycode for right"),
        None => Keycode::Right,
        _ => panic!("Malformed right value"),
    };

    let sdrop = match keys.get("sdrop") {
        Some(Value::String(s)) => Keycode::from_name(s).expect("Unable to find keycode for sdrop"),
        None => Keycode::Down,
        _ => panic!("Malformed sdrop value"),
    };

    let hdrop = match keys.get("hdrop") {
        Some(Value::String(s)) => Keycode::from_name(s).expect("Unable to find keycode for hdrop"),
        None => Keycode::Up,
        _ => panic!("Malformed hdrop value"),
    };

    let rrot = match keys.get("rrot") {
        Some(Value::String(s)) => Keycode::from_name(s).expect("Unable to find keycode for rrot"),
        None => Keycode::X,
        _ => panic!("Malformed rrot value"),
    };

    let lrot = match keys.get("lrot") {
        Some(Value::String(s)) => Keycode::from_name(s).expect("Unable to find keycode for lrot"),
        None => Keycode::Z,
        _ => panic!("Malformed lrot value"),
    };

    let hold = match keys.get("hold") {
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
