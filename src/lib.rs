use std::{
    fmt::Display,
    thread::sleep,
    time::{Duration, SystemTime},
};

use field::{Field, PieceType, RotationState, FIELD_VIS_HEIGHT, FIELD_VIS_WIDTH};
use input::{Key, KeyEvent, KeyState};
use rotations::get_coords;
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

fn draw_field(field: &mut Field, canvas: &mut Canvas<Window>) -> Result<(), MyErr> {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();

    canvas.set_draw_color(Color::GRAY);
    canvas.fill_rect(Rect::new(0, 0, 500, 150))?;

    let hold = field.get_hold_piece();
    let next = field.get_next_piece();
    for y in 0..2 {
        for x in 0..4 {
            if hold.is_some() && get_coords(hold.unwrap(), RotationState::None).contains(&(y, x)) {
                let color = if field.can_hold() {
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
            canvas.set_draw_color(field.get_cell_color(x + 2, y + 2));
            canvas.fill_rect(Rect::new(x as i32 * 50, 150 + y as i32 * 50, 50, 50))?;
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
        Keycode::LShift => Some(KeyEvent {
            key: Key::Hold,
            press,
        }),
        _ => None,
    }
}

pub fn run() -> Result<(), MyErr> {
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

        draw_field(&mut field, &mut canvas)?;

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
