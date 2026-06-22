use sdl3::event::Event;
use sdl3::pixels::Color;
use sdl3::surface::Surface;
use std::path::Path;

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // SDL_Init
    let sdl_context = sdl3::init()?;
    let video_subsystem = sdl_context.video()?;

    // SDL_CreateWindow
    let window = video_subsystem
        .window("SDL3 Tutorial: Hello SDL3", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()?;

    // SDL_LoadBMP
    let hello_world = Surface::load_bmp(Path::new("D:/Projects/SDL/01-hello-sdl3/hello-sdl3.bmp"))?;

    // SDL_PollEvent's underlying queue
    let mut event_pump = sdl_context.event_pump()?;

    // Main loop — 'running is a label so we can break from inside the for
    'running: loop {
        // Poll events first — borrows event_pump mutably, must finish before surface borrow
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                break 'running;
            }
        }

        // SDL_GetWindowSurface — borrows event_pump immutably, safe now that poll is done
        let mut screen = window.surface(&event_pump).map_err(|e| e.to_string())?;

        // SDL_FillSurfaceRect
        screen.fill_rect(None, Color::RGB(0xFF, 0xFF, 0xFF))?;

        // SDL_BlitSurface
        hello_world.blit(None, &mut screen, None)?;

        // SDL_UpdateWindowSurface
        screen.update_window()?;
    }

    // No close() needed — window, surface, sdl_context all drop here automatically
    Ok(())
}
