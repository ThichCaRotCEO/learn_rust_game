use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::render::BlendMode;
use sdl3::render::FRect;
use sdl3::{image::LoadSurface, surface::Surface};

const SCREEN_WIDTH: f32 = 640.0;
const SCREEN_HEIGHT: f32 = 480.0;

const COLOR_MAGNITUDES: [u8; 3] = [0x00, 0x7f, 0xFF];

const TEX_R: usize = 0;
const TEX_G: usize = 1;
const TEX_B: usize = 2;
const TEX_A: usize = 3;
const BG_R: usize = 4;
const BG_G: usize = 5;
const BG_B: usize = 6;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl3::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(
            "SDL Tutorial: Chroma Key",
            SCREEN_WIDTH as u32,
            SCREEN_HEIGHT as u32,
        )
        .position_centered()
        .build()?;

    let mut canvas = window.into_canvas();
    let texture_creator = canvas.texture_creator();

    // let bg_texture = texture_creator.load_texture("src/img/background.png")?;

    let mut surface = Surface::from_file("src/img/colors.png")?;
    surface.set_color_key(true, Color::RGB(0x00, 0xFF, 0xFF))?;
    // let arrow = texture_creator.create_texture_from_surface(&surface)?;
    let mut colors_texture = texture_creator.create_texture_from_surface(&surface)?;
    colors_texture.set_blend_mode(BlendMode::Blend);

    let q = colors_texture.query();
    let tex_w = q.width as f32;
    let tex_h = q.height as f32;

    let mut channels: [usize; 7] = [2, 2, 2, 2, 2, 2, 2];

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    let ch: Option<usize> = match key {
                        Keycode::A => Some(TEX_R),
                        Keycode::S => Some(TEX_G),
                        Keycode::D => Some(TEX_B),
                        Keycode::F => Some(TEX_A),
                        Keycode::Q => Some(BG_R),
                        Keycode::W => Some(BG_G),
                        Keycode::E => Some(BG_B),
                        _ => None,
                    };
                    if let Some(idx) = ch {
                        channels[idx] = (channels[idx] + 1) % COLOR_MAGNITUDES.len();
                    }
                }
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(
            COLOR_MAGNITUDES[channels[BG_R]],
            COLOR_MAGNITUDES[channels[BG_G]],
            COLOR_MAGNITUDES[channels[BG_B]],
        ));
        canvas.clear();

        colors_texture.set_color_mod(
            COLOR_MAGNITUDES[channels[TEX_R]],
            COLOR_MAGNITUDES[channels[TEX_G]],
            COLOR_MAGNITUDES[channels[TEX_B]],
        );
        colors_texture.set_alpha_mod(COLOR_MAGNITUDES[channels[TEX_A]]);

        // let bg_q = bg_texture.query();
        let dst = FRect::new(
            (SCREEN_WIDTH - tex_w) / 2.0,
            (SCREEN_HEIGHT - tex_h) / 2.0,
            tex_w,
            tex_h,
        );
        canvas.copy(&colors_texture, None, dst)?;

        canvas.present();
    }

    Ok(())
}
