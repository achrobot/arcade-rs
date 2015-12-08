// phi/mod.rs

use ::sdl2::render::Renderer;
use ::sdl2::timer;
use ::sdl2::rect::Rect as SdlRect;

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
        key_space: Space
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
}

impl<'window> Phi<'window> {
    fn new(events: Events, renderer: Renderer<'window>) -> Phi<'window> {
        ::sdl2_image::init(::sdl2_image::INIT_PNG);

        Phi {
            events: events,
            renderer: renderer,
        }
    }

    pub fn output_size(&self) -> (f64, f64) {
        let (w,h) = self.renderer.output_size().unwrap();
        (w as f64, h as f64)
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
