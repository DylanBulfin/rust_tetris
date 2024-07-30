use std::{
    fmt::Display,
    thread::sleep,
    time::{Duration, SystemTime},
};

use field::{Field, PieceType, FIELD_VIS_HEIGHT, FIELD_VIS_WIDTH};
use input::{Key, KeyEvent, KeyState};
use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window,
};

mod field;
mod input;
mod rotations;

#[derive(Debug)]
pub enum MyErr {
    Str(String),
}

impl From<String> for MyErr {
    fn from(value: String) -> Self {
        MyErr::Str(value)
    }
}

impl Display for MyErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyErr::Str(s) => f.write_str(&s),
        }
    }
}

fn draw_field(field: &Field, canvas: &mut Canvas<Window>) -> Result<(), MyErr> {
    for y in 0..FIELD_VIS_HEIGHT {
        for x in 0..FIELD_VIS_WIDTH {
            canvas.set_draw_color(field.get_cell_color(x + 2, y + 2));
            canvas.fill_rect(Rect::new(x as i32 * 50, y as i32 * 50, 50, 50))?;
        }
    }

    Ok(())
}

fn process_keycode(kc: Keycode, press: bool, field: &mut Field) -> Option<KeyEvent> {
    match kc {
        Keycode::Up => Some(KeyEvent {
            key: Key::HDrop,
            press,
        }),
        Keycode::Down => Some(KeyEvent {
            key: Key::SDrop,
            press,
        }),
        Keycode::Right => Some(KeyEvent {
            key: Key::Right,
            press,
        }),
        Keycode::Left => Some(KeyEvent {
            key: Key::Left,
            press,
        }),
        Keycode::X => Some(KeyEvent {
            key: Key::RRot,
            press,
        }),
        Keycode::Z => Some(KeyEvent {
            key: Key::LRot,
            press,
        }),

        _ => None,
    }
}

pub fn run() -> Result<(), MyErr> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("tet.rs", 500, 1000)
        .position_centered()
        .always_on_top()
        .build()
        .expect("Unable to initialize window");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Unable to create canvas");
    let mut field = Field::new();
    let mut keys = KeyState::new();

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
                    } => {
                        process_keycode(kc, true, &mut field).map(|e| keys.update(e, &mut field));
                        println!("Shit");
                    }
                    Event::KeyUp {
                        keycode: Some(kc),
                        repeat: false,
                        ..
                    } => {
                        process_keycode(kc, false, &mut field).map(|e| keys.update(e, &mut field));
                    }
                    _ => (),
                };
            }
        }

        keys.handle_dirs(&mut field);

        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();

        draw_field(&field, &mut canvas)?;

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

    Err(MyErr::Str("Broke".to_string()))
}
