use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::render::FRect;
use std::time::Instant;

const SCREEN_WIDTH: f32 = 640.0;
const SCREEN_HEIGHT: f32 = 480.0;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl3::init()?;
    let video_subsystem = sdl_context.video()?;
    let ttf_context = sdl3::ttf::init()?;

    let window = video_subsystem
        .window(
            "SDL Tutorial: Timing",
            SCREEN_WIDTH as u32,
            SCREEN_HEIGHT as u32,
        )
        .position_centered()
        .build()?;

    let mut canvas = window.into_canvas();
    let texture_creator = canvas.texture_creator();

    // let bg_texture = texture_creator.load_texture("src/img/background.png")?;
    // let font = ttf_context.load_font("src/font/lazy.ttf", 28.0)?;
    let font = ttf_context.load_font("src/font/momo.ttf", 28.0)?;

    let mut start_time: Option<Instant> = None;

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Return),
                    ..
                } => {
                    start_time = Some(Instant::now());
                }
                _ => {}
            }
        }

        let text = match start_time {
            Some(t) => format!("Miliseconds since start time {}", t.elapsed().as_millis()),
            None => "Press enter to start".to_string(),
        };

        let text_surface = font.render(&text).blended(Color::RGB(0x00, 0x00, 0x00))?;
        let text_texture = texture_creator.create_texture_from_surface(&text_surface)?;

        let q = text_texture.query();
        let tex_w = q.width as f32;
        let tex_h = q.height as f32;

        canvas.set_draw_color(Color::RGB(0xFF, 0xFF, 0xFF));
        canvas.clear();

        // let bg_q = bg_texture.query();
        let dst = FRect::new(
            (SCREEN_WIDTH - tex_w) / 2.0,
            (SCREEN_HEIGHT - tex_h) / 2.0,
            tex_w,
            tex_h,
        );
        canvas.copy(&text_texture, None, dst)?;

        canvas.present();
    }

    Ok(())
}
