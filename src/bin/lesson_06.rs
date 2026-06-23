use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::render::FPoint;
use sdl3::render::FRect;
// use sdl3::render::FlipMode;
use sdl3::{image::LoadSurface, surface::Surface};

const SCREEN_WIDTH: f32 = 640.0;
const SCREEN_HEIGHT: f32 = 480.0;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl3::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(
            "SDL Tutorial: Rotation & Flipping",
            SCREEN_WIDTH as u32,
            SCREEN_HEIGHT as u32,
        )
        .position_centered()
        .build()?;

    let mut canvas = window.into_canvas();
    let texture_creator = canvas.texture_creator();

    // let bg_texture = texture_creator.load_texture("src/img/background.png")?;

    let mut surface = Surface::from_file("src/img/arrow.png")?;
    surface.set_color_key(true, Color::RGB(0x00, 0xFF, 0xFF))?;
    let arrow = texture_creator.create_texture_from_surface(&surface)?;

    let q = arrow.query();
    let arrow_w = q.width as f32;
    let arrow_h = q.height as f32;

    let mut event_pump = sdl_context.event_pump()?;
    let mut degrees: f64 = 0.0;
    // let mut flip = FlipMode::None;
    let mut flip_v: bool = false;
    let mut flip_h: bool = false;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(key), ..
                } => match key {
                    Keycode::Right => degrees += 36.0,
                    Keycode::Left => degrees -= 36.0,
                    Keycode::_1 => {
                        flip_h = true;
                        flip_v = false;
                    }
                    Keycode::_2 => {
                        flip_h = false;
                        flip_v = false;
                    }
                    Keycode::_3 => {
                        flip_h = false;
                        flip_v = true;
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0xFF, 0xFF, 0xFF));
        canvas.clear();

        // let bg_q = bg_texture.query();
        let dst = FRect::new(
            (SCREEN_WIDTH - arrow_w) / 2.0,
            (SCREEN_HEIGHT - arrow_h) / 2.0,
            arrow_w,
            arrow_h,
        );

        let center = FPoint::new(arrow_w / 2.0, arrow_h / 2.0);
        canvas.copy_ex(&arrow, None, dst, degrees, center, flip_h, flip_v)?;

        canvas.present();
    }

    Ok(())
}
