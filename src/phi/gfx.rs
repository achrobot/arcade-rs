use ::phi::data::Rectangle;
use ::std::cell::RefCell;
use ::std::path::Path;
use ::std::rc::Rc;
use ::sdl2::render::{Renderer, Texture};
use ::sdl2_image::LoadTexture;

// Common interface for rendering a graphical component
// to an area of the current window
pub trait Renderable {
    fn render(&self, renderer: &mut Renderer, dest: Rectangle);
}

pub trait CopySprite<T> {
    fn copy_sprite(&mut self, sprite: &T, dest: Rectangle);
}

impl <'window, T: Renderable> CopySprite<T> for Renderer<'window> {
    fn copy_sprite(&mut self, renderable: &T, dest: Rectangle) {
        renderable.render(self, dest);
    }
}

#[derive(Clone)]
pub struct Sprite {
  tex: Rc<RefCell<Texture>>,
  src: Rectangle,
}

impl Sprite {
    // Creates a new Sprite wrapping a texture
    pub fn new(texture: Texture) -> Sprite {
        let tex_query = texture.query();

        Sprite {
            tex: Rc::new(RefCell::new(texture)),
            src: Rectangle {
                w: tex_query.width as f64,
                h: tex_query.height as f64,
                x: 0.0,
                y: 0.0,
            }
        }
    }

    pub fn load(renderer: &Renderer, path: &str) -> Option<Sprite> {
        renderer.load_texture(Path::new(path)).ok().map(Sprite::new)
    }

    pub fn region(&self, rect: Rectangle) -> Option<Sprite> {
        let new_src = Rectangle {
            x: rect.x + self.src.x,
            y: rect.y + self.src.y,
            ..rect
        };

        // Verify that the requested region is inside the current one
        if self.src.contains(new_src) {
            Some(Sprite {
                tex: self.tex.clone(),
                src: new_src,
            })
        }
        else {
            None
        }
    }

    pub fn size(&self) -> (f64, f64) {
        (self.src.w, self.src.h)
    }
}

impl Renderable for Sprite {

    // Render the sprite to the current window
    fn render(&self, renderer: &mut Renderer, dest: Rectangle) {
        renderer.copy(&mut self.tex.borrow_mut(), self.src.to_sdl(), dest.to_sdl());
    }
}

pub struct AnimatedSprite {
    // The fraemes that will be rendered, in order
    sprites: Rc<Vec<Sprite>>,

    // The time it takes to get from one frame to the next, in seconds
    frame_delay: f64,

    // The total time that the sprite has been alive, in seconds,
    // from which the current frae is derived
    current_time: f64,
}

impl AnimatedSprite {
    // Create a new animated sprite with a current time of 0.
    pub fn new(sprites: Vec<Sprite>, frame_delay: f64) -> AnimatedSprite {
        AnimatedSprite {
            sprites: Rc::new(sprites),
            frame_delay: frame_delay,
            current_time: 0.0,
        }
    }

    // Create a new animated sprite that goes to a next frame 'fps' times every second
    pub fn new_with_fps(sprites: Vec<Sprite>, fps: f64) -> AnimatedSprite {
        if fps == 0.0 {
            panic!("FPS of 0.0 is invalid.");
        }
        AnimatedSprite::new(sprites, 1.0 / fps)
    }

    // Return the number of frames in this animation
    pub fn frame_count(&self) -> usize {
        self.sprites.len()
    }

    // Set the time it takes to get from one frame to the next, in seconds.
    // If the value is negative, we 'rewind' the animation
    pub fn set_frame_delay(&mut self, frame_delay: f64) {
        self.frame_delay = frame_delay;
    }

    // Set the number of frames the animation goes through every second.
    // If the value is negative, we 'rewind' the animation
    pub fn set_fps(&mut self, fps: f64) {
        if fps == 0.0 {
            panic!("FPS of 0.0 is invalid.");
        }
        self.set_frame_delay(1.0 / fps);
    }

    // Adds seconds to the current time of the anmited sprite,
    // so that it knows when to go to the next frame.
    pub fn add_time(&mut self, dt: f64) {
        self.current_time += dt;

        // Handle 'rewinding' animations
        if self.current_time < 0.0 {
            self.current_time = (self.frame_count() - 1) as f64 * self.frame_delay;
        }
    }

}

impl Renderable for AnimatedSprite {

    // Render the current frame of the sprite
    fn render(&self, renderer: &mut Renderer, dest: Rectangle) {
        let current_frame =
            (self.current_time / self.frame_delay) as usize % self.frame_count();

        let sprite = &self.sprites[current_frame];
        sprite.render(renderer, dest);
    }
}
