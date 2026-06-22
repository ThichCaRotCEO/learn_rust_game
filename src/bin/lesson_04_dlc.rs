use sdl3::event::Event;
use sdl3::pixels::Color;
use sdl3::{
    image::{LoadSurface, LoadTexture},
    render::FRect,
    surface::Surface,
};

const SCREEN_WIDTH: u32 = 3840;
const SCREEN_HEIGHT: u32 = 2160;
const RENDER_WIDTH: f32 = 320.0;
const RENDER_HEIGHT: f32 = 240.0;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl3::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(
            "SDL Tutorial: Color Keying DLC",
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
        )
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

        // let bg_q = bg_texture.query();
        canvas.copy(
            &bg_texture,
            None,
            FRect::new(0.0, 0.0, SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32),
        )?;

        // let foo_q = foo_texture.query();
        canvas.copy(
            &foo_texture,
            None,
            FRect::new(
                (SCREEN_WIDTH as f32 - RENDER_WIDTH) / 2.0,
                (SCREEN_HEIGHT as f32 - RENDER_HEIGHT) / 2.0,
                RENDER_WIDTH,
                RENDER_HEIGHT,
            ),
        )?;

        canvas.present();
    }

    Ok(())
}
