use std::{
    fmt::Display,
    io,
    thread::sleep,
    time::{Duration, SystemTime},
};

use config::{get_config, Config};
use input::{Key, KeyEvent, KeyState};
use rotations::get_coords;
use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window,
};
use state::{RotationState, State, FIELD_VIS_HEIGHT, FIELD_VIS_WIDTH};

mod config;
mod input;
mod rotations;
mod state;

#[derive(Debug)]
pub enum TetrErr {
    Str(String),
    IOError(io::Error),
}

impl From<String> for TetrErr {
    fn from(value: String) -> Self {
        TetrErr::Str(value)
    }
}

impl From<io::Error> for TetrErr {
    fn from(value: io::Error) -> Self {
        TetrErr::IOError(value)
    }
}

impl Display for TetrErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TetrErr::Str(s) => f.write_str(&s),
            TetrErr::IOError(e) => f.write_fmt(format_args!("{}", e)),
        }
    }
}

fn draw_field(state: &mut State, canvas: &mut Canvas<Window>) -> Result<(), TetrErr> {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();

    canvas.set_draw_color(Color::GRAY);
    canvas.fill_rect(Rect::new(0, 0, 500, 150))?;

    let hold = state.get_hold_piece();
    let next = state.get_next_piece();
    for y in 0..2 {
        for x in 0..4 {
            if hold.is_some() && get_coords(hold.unwrap(), RotationState::None).contains(&(y, x)) {
                let color = if state.can_hold() {
                    hold.unwrap().into()
                } else {
                    Color::BLACK
                };
                canvas.set_draw_color::<Color>(color);
                canvas.fill_rect(Rect::new(x as i32 * 50, y as i32 * 50, 50, 50))?;
            }
            if get_coords(next, RotationState::None).contains(&(y, x)) {
                canvas.set_draw_color::<Color>(next.into());
                canvas.fill_rect(Rect::new(300 + x as i32 * 50, y as i32 * 50, 50, 50))?;
            }
        }
    }

    for y in 0..FIELD_VIS_HEIGHT {
        for x in 0..FIELD_VIS_WIDTH {
            canvas.set_draw_color(state.get_cell_color(x + 2, y + 2));
            canvas.fill_rect(Rect::new(x as i32 * 50, 150 + y as i32 * 50, 50, 50))?;
        }
    }

    Ok(())
}

fn key_from_keycode(kc: Keycode, config: &Config) -> Option<Key> {
    if kc == config.keys().left() {
        Some(Key::Left)
    } else if kc == config.keys().right() {
        Some(Key::Right)
    } else if kc == config.keys().sdrop() {
        Some(Key::SDrop)
    } else if kc == config.keys().hdrop() {
        Some(Key::HDrop)
    } else if kc == config.keys().rrot() {
        Some(Key::RRot)
    } else if kc == config.keys().lrot() {
        Some(Key::LRot)
    } else if kc == config.keys().hold() {
        Some(Key::Hold)
    } else {
        None
    }
}

pub fn run() -> Result<(), TetrErr> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("tet.rs", 500, 1150)
        .position_centered()
        .always_on_top()
        .build()
        .expect("Unable to initialize window");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Unable to create canvas");
    let config = get_config();
    let mut state = State::new();
    let mut keys = KeyState::new(config);

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        let timer = SystemTime::now();
        while SystemTime::now()
            .duration_since(timer)
            .expect("Couldn't do timing right")
            < Duration::new(0, 1_000_000_000u32 / 45)
        //TODO Fix this being hardcoded
        {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    Event::KeyDown {
                        keycode: Some(kc),
                        repeat: false,
                        ..
                    } => match key_from_keycode(kc, &config) {
                        Some(k) => keys.update(
                            KeyEvent {
                                key: k,
                                press: true,
                            },
                            &mut state,
                        ),
                        None => (),
                    },
                    Event::KeyUp {
                        keycode: Some(kc),
                        repeat: false,
                        ..
                    } => match key_from_keycode(kc, &config) {
                        Some(k) => keys.update(
                            KeyEvent {
                                key: k,
                                press: false,
                            },
                            &mut state,
                        ),
                        None => (),
                    },
                    _ => (),
                };
            }
        }

        keys.handle_special(&mut state);

        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();

        draw_field(&mut state, &mut canvas)?;

        canvas.present();

        match SystemTime::now().duration_since(timer) {
            Ok(d) => {
                if d > Duration::new(0, 1_000_000_000) {
                    sleep(Duration::new(0, 1_000_000_000u32 / 60) - d)
                }
            }
            Err(_) => (),
        }
    }

    Err(TetrErr::Str("Broke".to_string()))
}
