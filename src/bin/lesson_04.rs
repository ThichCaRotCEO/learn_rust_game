use sdl3::event::Event;
use sdl3::pixels::Color;
use sdl3::{
    image::{LoadSurface, LoadTexture},
    render::FRect,
    surface::Surface,
};

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl3::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("SDL Tutorial: Color Keying", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()?;

    let mut canvas = window.into_canvas();
    let texture_creator = canvas.texture_creator();

    let bg_texture = texture_creator.load_texture("src/img/background.png")?;

    let mut foo_surface = Surface::from_file("src/img/foo.png")?;
    foo_surface.set_color_key(true, Color::RGB(0x00, 0xFF, 0xFF))?;
    let foo_texture = texture_creator.create_texture_from_surface(&foo_surface)?;

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                break 'running;
            }
        }

        canvas.set_draw_color(Color::RGB(0xFF, 0xFF, 0xFF));
        canvas.clear();

        let bg_q = bg_texture.query();
        canvas.copy(
            &bg_texture,
            None,
            FRect::new(0.0, 0.0, bg_q.width as f32, bg_q.height as f32),
        )?;

        let foo_q = foo_texture.query();
        canvas.copy(
            &foo_texture,
            None,
            FRect::new(240.0, 190.0, foo_q.width as f32, foo_q.height as f32),
        )?;

        canvas.present();
    }

    Ok(())
}
