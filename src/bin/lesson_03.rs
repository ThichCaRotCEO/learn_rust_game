use sdl3::event::Event;
use sdl3::image::LoadTexture;
use sdl3::keyboard::{Keycode, Scancode};
use sdl3::pixels::Color;
use sdl3::render::FRect;

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

enum Direction {
    Up,
    Down,
    Right,
    Left,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl3::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("SDL Tutorial: Key Presses", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()?;

    let mut canvas = window.into_canvas();
    let texture_creator = canvas.texture_creator();

    let up_textture = texture_creator.load_texture("src/img/up.png")?;
    let down_textture = texture_creator.load_texture("src/img/down.png")?;
    let right_textture = texture_creator.load_texture("src/img/right.png")?;
    let left_textture = texture_creator.load_texture("src/img/left.png")?;

    let mut event_pump = sdl_context.event_pump()?;
    let mut current = Direction::Up;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(key), ..
                } => match key {
                    Keycode::Up => current = Direction::Up,
                    Keycode::Down => current = Direction::Down,
                    Keycode::Right => current = Direction::Right,
                    Keycode::Left => current = Direction::Left,
                    _ => {}
                },
                _ => {}
            }
        }

        let keys = event_pump.keyboard_state();
        let bg = if keys.is_scancode_pressed(Scancode::Up) {
            Color::RGB(0xFF, 0x00, 0x00)
        } else if keys.is_scancode_pressed(Scancode::Down) {
            Color::RGB(0x00, 0xFF, 0x00)
        } else if keys.is_scancode_pressed(Scancode::Right) {
            Color::RGB(0xFF, 0xFF, 0x00)
        } else if keys.is_scancode_pressed(Scancode::Left) {
            Color::RGB(0x00, 0x00, 0xFF)
        } else {
            Color::RGB(0xFF, 0xFF, 0xFF)
        };

        canvas.set_draw_color(bg);
        canvas.clear();

        let texture = match current {
            Direction::Up => &up_textture,
            Direction::Down => &down_textture,
            Direction::Right => &right_textture,
            Direction::Left => &left_textture,
        };

        let q = texture.query();
        let dst = FRect::new(
            (SCREEN_WIDTH as f32 - q.width as f32) / 2.0,
            (SCREEN_HEIGHT as f32 - q.height as f32) / 2.0,
            q.width as f32,
            q.height as f32,
        );

        canvas.copy(texture, None, dst)?;
        canvas.present();
    }

    Ok(())
}
