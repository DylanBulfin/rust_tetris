use std::time::Duration;

use field::{Field, FIELD_HEIGHT, FIELD_VIS_HEIGHT, FIELD_WIDTH};
use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window,
};

mod field;
mod rotations;

fn draw_field(field: &Field, canvas: &mut Canvas<Window>) -> Result<(), String> {
    for y in 0..FIELD_VIS_HEIGHT {
        for x in 0..FIELD_WIDTH {
            canvas.set_draw_color(field.get_cell_color(x, y + 2));
            canvas.fill_rect(Rect::new(x as i32 * 50, y as i32 * 50, 50, 50))?;
        }
    }

    Ok(())
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-sdl2 demo", 500, 1000)
        .position_centered()
        .always_on_top()
        .build()
        .expect("Unable to initialize window");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Unable to create canvas");
    let mut field = Field::new();

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => field.piece_down(),
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => field.piece_left(),
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => field.piece_right(),
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();

        draw_field(&field, &mut canvas)?;

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Err("Broke".to_string())
}