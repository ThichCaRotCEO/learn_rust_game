use sdl3::event::Event;
use sdl3::pixels::Color;
use sdl3::render::FRect;
use sdl3::{image::LoadSurface, surface::Surface};

const SCREEN_WIDTH: f32 = 640.0;
const SCREEN_HEIGHT: f32 = 480.0;
const BUTTON_WIDTH: f32 = 300.0;
const BUTTON_HEIGHT: f32 = 200.0;

#[derive(Clone, Copy)]
enum ButtonSprite {
    MouseOut = 0,
    MouseOverMotion = 1,
    MouseDown = 2,
    MouseUp = 3,
}

struct Button {
    x: f32,
    y: f32,
    sprite: ButtonSprite,
}

impl Button {
    fn new(x: f32, y: f32) -> Self {
        Button {
            x,
            y,
            sprite: ButtonSprite::MouseOut,
        }
    }

    fn handle_mouse(&mut self, mx: f32, my: f32, kind: MouseKind) {
        let inside = mx >= self.x
            && mx < self.x + BUTTON_WIDTH
            && my >= self.y
            && my < self.y + BUTTON_HEIGHT;

        self.sprite = if !inside {
            ButtonSprite::MouseOut
        } else {
            match kind {
                MouseKind::Motion => ButtonSprite::MouseOverMotion,
                MouseKind::Down => ButtonSprite::MouseDown,
                MouseKind::Up => ButtonSprite::MouseUp,
            }
        };
    }

    fn sprite_clip(&self) -> FRect {
        FRect::new(
            0.0,
            self.sprite as u8 as f32 * BUTTON_HEIGHT,
            BUTTON_WIDTH,
            BUTTON_HEIGHT,
        )
    }

    fn dst_rect(&self) -> FRect {
        FRect::new(self.x, self.y, BUTTON_WIDTH, BUTTON_HEIGHT)
    }
}

enum MouseKind {
    Motion,
    Down,
    Up,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl3::init()?;
    let video_subsystem = sdl_context.video()?;

    // let ttf_context = sdl3::ttf::init()?;

    let window = video_subsystem
        .window(
            "SDL Tutorial: Mouse Events",
            SCREEN_WIDTH as u32,
            SCREEN_HEIGHT as u32,
        )
        .position_centered()
        .build()?;

    let mut canvas = window.into_canvas();
    let texture_creator = canvas.texture_creator();

    // let bg_texture = texture_creator.load_texture("src/img/background.png")?;
    // let font = ttf_context.load_font("src/font/lazy.ttf", 28.0)?;
    // let font = ttf_context.load_font("src/font/momo.ttf", 28.0)?;
    let mut button_surface = Surface::from_file("src/img/button.png")?;
    button_surface.set_color_key(true, Color::RGB(0x00, 0xFF, 0xFF))?;
    let button_texture = texture_creator.create_texture_from_surface(&button_surface)?;

    let mut buttons = [
        Button::new(0.0, 0.0),
        Button::new(SCREEN_WIDTH - BUTTON_WIDTH, 0.0),
        Button::new(0.0, SCREEN_HEIGHT - BUTTON_HEIGHT),
        Button::new(SCREEN_WIDTH - BUTTON_WIDTH, SCREEN_HEIGHT - BUTTON_HEIGHT),
    ];

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,

                Event::MouseMotion { x, y, .. } => {
                    for btn in buttons.iter_mut() {
                        btn.handle_mouse(x, y, MouseKind::Motion);
                    }
                }

                Event::MouseButtonDown { x, y, .. } => {
                    for btn in buttons.iter_mut() {
                        btn.handle_mouse(x, y, MouseKind::Down);
                    }
                }

                Event::MouseButtonUp { x, y, .. } => {
                    for btn in buttons.iter_mut() {
                        btn.handle_mouse(x, y, MouseKind::Up);
                    }
                }

                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0xFF, 0xFF, 0xFF));
        canvas.clear();

        // let bg_q = bg_texture.query();
        for btn in buttons.iter() {
            canvas.copy(&button_texture, btn.sprite_clip(), btn.dst_rect())?;
        }

        canvas.present();
    }

    Ok(())
}
