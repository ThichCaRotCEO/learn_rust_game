use sdl3::event::Event;
use sdl3::pixels::Color;
use sdl3::{image::LoadSurface, render::FRect, surface::Surface};

const SCREEN_WIDTH: f32 = 640.0;
const SCREEN_HEIGHT: f32 = 480.0;
const SPRITE_SIZE: f32 = 100.0;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl3::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(
            "SDL Tutorial: Sprite Clipping Stretching",
            SCREEN_WIDTH as u32,
            SCREEN_HEIGHT as u32,
        )
        .position_centered()
        .build()?;

    let mut canvas = window.into_canvas();
    let texture_creator = canvas.texture_creator();

    // let bg_texture = texture_creator.load_texture("src/img/background.png")?;

    let mut surface = Surface::from_file("src/img/dots.png")?;
    surface.set_color_key(true, Color::RGB(0x00, 0xFF, 0xFF))?;
    let sheet = texture_creator.create_texture_from_surface(&surface)?;

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                break 'running;
            }
        }

        canvas.set_draw_color(Color::RGB(0xFF, 0xFF, 0xFF));
        canvas.clear();

        // let bg_q = bg_texture.query();
        canvas.copy(
            &sheet,
            FRect::new(0.0, 0.0, SPRITE_SIZE, SPRITE_SIZE),
            FRect::new(0.0, 0.0, SPRITE_SIZE, SPRITE_SIZE),
        )?;

        let half = SPRITE_SIZE * 0.5;
        canvas.copy(
            &sheet,
            FRect::new(SPRITE_SIZE, 0.0, SPRITE_SIZE, SPRITE_SIZE),
            FRect::new(SPRITE_SIZE - half, 0.0, half, half),
        )?;

        let double = SPRITE_SIZE * 2.0;
        canvas.copy(
            &sheet,
            FRect::new(0.0, SPRITE_SIZE, SPRITE_SIZE, SPRITE_SIZE),
            FRect::new(0.0, SCREEN_HEIGHT - double, double, double),
        )?;

        let squished_h = SPRITE_SIZE * 0.5;
        canvas.copy(
            &sheet,
            FRect::new(SPRITE_SIZE, SPRITE_SIZE, SPRITE_SIZE, SPRITE_SIZE),
            FRect::new(
                SCREEN_WIDTH - SPRITE_SIZE,
                SCREEN_HEIGHT - squished_h,
                SPRITE_SIZE,
                squished_h,
            ),
        )?;

        canvas.present();
    }

    Ok(())
}
