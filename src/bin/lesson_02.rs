use sdl3::event::Event;
use sdl3::image::LoadTexture;
use sdl3::pixels::Color;
use sdl3::render::FRect;

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl3::init()?;
    let video_subsystem = sdl_context.video()?;

    // SDL_CreateWindowAndRenderer equivalent
    let window = video_subsystem
        .window(
            "SDL3 Tutorial: Textures and Extension Libraries",
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
        )
        .position_centered()
        .build()?;

    let mut canvas = window.into_canvas();

    // TextureCreator must outlive every texture it creates — this replaces the
    // gRenderer pointer that LTexture::loadFromFile received in C++
    let texture_creator = canvas.texture_creator();

    // IMG_Load + SDL_CreateTextureFromSurface, in one call
    // Texture drops automatically when it goes out of scope — no destroy() needed
    let texture = texture_creator.load_texture(
        "D:/Projects/SDL/Learning/Learning_01/x64/Debug/lazyfoo-rust/src/img/loaded.png",
    )?;

    // SDL_QueryTexture equivalent — get dimensions for rendering
    let query = texture.query();
    let (tex_width, tex_height) = (query.width, query.height);

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                break 'running;
            }
        }

        // SDL_SetRenderDrawColor + SDL_RenderClear
        canvas.set_draw_color(Color::RGB(0xFF, 0xFF, 0xFF));
        canvas.clear();

        // SDL_RenderTexture — float rect matches SDL3's SDL_FRect
        let dst = FRect::new(0.0, 0.0, tex_width as f32, tex_height as f32);
        canvas.copy(&texture, None, dst)?;

        // SDL_RenderPresent
        canvas.present();
    }

    Ok(())
}
