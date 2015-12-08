// phi/mod.rs

use self::gfx::Sprite;
use ::sdl2::render::Renderer;
use ::sdl2::pixels::Color;
use ::sdl2::timer;
use ::sdl2::rect::Rect as SdlRect;
use ::std::collections::HashMap;
use ::std::path::Path;

#[macro_use]
mod events;
pub mod data;
pub mod gfx;

struct_events! {
    keyboard: {
        key_escape : Escape,
        key_up: Up,
        key_down: Down,
        key_left: Left,
        key_right: Right,
        key_space: Space,
        key_return: Return
    },

    else: {
        quit: Quit { .. }
    }
}

/// Bundles the Phi abstraction in a single structure which can
/// be passed easily between functions.
pub struct Phi<'window> {
    pub events: Events,
    pub renderer: Renderer<'window>,

    cached_fonts: HashMap<(&'static str, i32), ::sdl2_ttf::Font>,
}

impl<'window> Phi<'window> {
    fn new(events: Events, renderer: Renderer<'window>) -> Phi<'window> {
        ::sdl2_image::init(::sdl2_image::INIT_PNG);

        Phi {
            events: events,
            renderer: renderer,
            cached_fonts: HashMap::new(),
        }
    }

    pub fn output_size(&self) -> (f64, f64) {
        let (w,h) = self.renderer.output_size().unwrap();
        (w as f64, h as f64)
    }

    pub fn ttf_str_sprite(&mut self, text: &str, font_path: &'static str, size:i32, color: Color) -> Option<Sprite> {
        // First we determine whether the Font is cached - if so, use it
        if let Some(font) = self.cached_fonts.get(&(font_path, size)) {
            return font.render(text, ::sdl2_ttf::blended(color)).ok()
                // If this worked be try to make this surface into a texture
                .and_then(|surface| self.renderer.create_texture_from_surface(&surface).ok())
                // If this worked we load
                .map(Sprite::new)
        }
        // Otherwise try to load the requested font
        ::sdl2_ttf::Font::from_file(Path::new(font_path), size).ok()
            // We must wrap the next steps in a closure because Borrow Checker
            .and_then(|font| {
                // If this works we cached the font we loaded
                self.cached_fonts.insert((font_path, size), font);
                // Then we call this method recursively
                self.ttf_str_sprite(text, font_path, size, color)
            })
    }
}

impl<'window> Drop for Phi<'window> {
    fn drop(&mut self) {
        ::sdl2_image::quit();
    }
}

/// A ViewAction is a way for the currently executed view to communicate
/// with the game loop. It specifies which action should be executed
/// before the next rendering.
pub enum ViewAction {
    None,
    Quit,
    ChangeView(Box<View>),
}

pub trait View {

    /// Called when self becomes main, rendered view.
    fn resume(&mut self, _context: &mut Phi) {
    }

    /// Called when self stops being main, rendered view.
    fn pause(&mut self, _context: &mut Phi) {
    }

    /// Called every frame to take care of both the logic and
    /// the rendering of the view; expressed in seconds.
    fn render(&mut self, context: &mut Phi, elapsed: f64) -> ViewAction;
}


/// Create a window with a name 'title', initialize the underlying
/// libraries, and start the game with the 'View' returned by 'init()'.
///
pub fn spawn<F>(title: &str, init: F)
        where F: Fn(&mut Phi) -> Box<View> {

    // Initialize SDL2
    let     sdl_context = ::sdl2::init().unwrap();
    let mut sdl_timer = sdl_context.timer().unwrap();
    let     sdl_video = sdl_context.video().unwrap();
    let     _ttf_context = ::sdl2_ttf::init().unwrap();

    // Create the window
    let window = sdl_video.window("ArcadeRS Shooter", 800, 600)
            .position_centered().opengl().resizable()
            .build().unwrap();

    let mut context = Phi::new(
        Events::new(sdl_context.event_pump().unwrap() ),
        window.renderer()
            .accelerated()
            .build().unwrap() );

    let mut current_view = init(&mut context);
    //: Box<::phi::View> =
    //        Box::new(::views::DefaultView );

    current_view.resume(&mut context);

    // Frame timing stuff
    let interval = 1_000 / 60;
    let mut before = sdl_timer.ticks();
    let mut last_second = sdl_timer.ticks();
    let mut fps = 0u16;

    loop {

        // Frame timing stuff . . .

        let now = sdl_timer.ticks();
        let dt = now - before;
        let elapsed = dt as f64 / 1_000.0;

        // Wait a bit if the frame has come too fast
        if dt < interval {
            sdl_timer.delay(interval - dt);
            continue;
        }

        before = now;
        fps += 1;

        if now - last_second > 1_000 {
            println!("FPS: {}", fps);
            last_second = now;
            fps = 0;
        }

        // Logic and rendering . . .

        context.events.pump(&mut context.renderer);

        match current_view.render(&mut context, elapsed) {

            ViewAction::None => context.renderer.present(),

            ViewAction::Quit => {
                current_view.pause(&mut context);
                break;
            },

            ViewAction::ChangeView(new_view) => {
                current_view.pause(&mut context);
                current_view = new_view;
                current_view.resume(&mut context);
            }
        }

    }
}
