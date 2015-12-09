// shared.rs

use ::sdl2::render::Renderer;
use ::phi::data::Rectangle;
use ::phi::gfx::{Renderable, Sprite};

#[derive(Clone)]
pub struct Background {
    pub pos: f64,
    // The number of pixels to move left every second
    pub vel: f64,
    pub sprite: Sprite,
}

impl Background {
    pub fn render(&mut self, renderer: &mut Renderer, elapsed: f64) {
        // We define a logical position as depending solely on the time
        // and on the dimensions of the image, not on the screen size.
        let size = self.sprite.size();
        self.pos += self.vel * elapsed;
        if self.pos > size.0 {
            self.pos -= size.0;
        }

        // We determine the scale ration of the window to the sprite.
        let (win_w, win_h) = renderer.output_size().unwrap();
        let scale = win_h as f64 / size.1;

        // We render as many copies of the background as necessary
        // to fill the screen.
        let mut physical_left = -self.pos * scale;

        while physical_left < win_w as f64 {
            // While the left of the image is still inside the window
            self.sprite.render(renderer, Rectangle {
            //renderer.copy_sprite(&self.sprite, Rectangle {
                x: physical_left,
                y: 0.0,
                w: size.0 * scale,
                h: win_h as f64,
            });

            physical_left += size.0 * scale;
        }
    }
}

// A group of backgrounds that can be passed from view to view
#[derive(Clone)]
pub struct Backgrounds {
    pub back: Background,
    pub middle: Background,
    pub front: Background,
}

impl Backgrounds {
    pub fn new(renderer: &mut Renderer) -> Backgrounds {
        Backgrounds {
            back: Background {
                pos: 0.0,
                vel: 20.0,
                sprite: Sprite::load(renderer, "assets/starBG.png").unwrap(),
            },
            middle: Background {
                pos: 0.0,
                vel: 40.0,
                sprite: Sprite::load(renderer, "assets/starMG.png").unwrap(),
            },
            front: Background {
                pos: 0.0,
                vel: 80.0,
                sprite: Sprite::load(renderer, "assets/starFG.png").unwrap(),
            },
        }
    }
}
