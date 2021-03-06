// views/mod.rs

pub mod game;
pub mod main_menu;
pub mod shared;

/*
use ::std::path::Path;

use ::phi::{Phi, View, ViewAction};
use ::phi::data::Rectangle;
use ::phi::gfx::Sprite;
use ::sdl2::render::Renderer;
use ::sdl2::pixels::Color;
//use ::sdl2::rect::Rect as SdlRect;
use ::sdl2::render::{Texture, TextureQuery};
use ::sdl2_image::LoadTexture;

// CONSTANTS . . .

const DEBUG: bool = false;

/// Pixels travelled by a player ship every second, when it is moving
const PLAYER_SPEED: f64 = 180.0;

const SHIP_W: f64 = 43.0;
const SHIP_H: f64 = 39.0;

// DATA TYPES . . .

#[derive(Clone, Copy)]
enum ShipFrame {
    UpNorm   = 0,
    UpFast   = 1,
    UpSlow   = 2,
    MidNorm  = 3,
    MidFast  = 4,
    MidSlow  = 5,
    DownNorm = 6,
    DownFast = 7,
    DownSlow = 8,
}

struct Ship {

    rect: Rectangle,
    sprites: Vec<Sprite>,
    current: ShipFrame,
}

#[derive(Clone)]
struct Background {
    pos: f64,
    // The number of pixels to move left every second
    vel: f64,
    sprite: Sprite,
}

// VIEW DEFINITIONS . . .

pub struct ShipView {

    player: Ship,
    bg_back: Background,
    bg_middle: Background,
    bg_front: Background,
}

impl ShipView {

    pub fn new(phi: &mut Phi) -> ShipView {

        // Load the texture from the filesystem
        let spritesheet = Sprite::load(&mut phi.renderer, "assets/spaceship.png").unwrap();
        let mut sprites = Vec::with_capacity(9);
        for y in 0..3 {
            for x in 0..3 {
                sprites.push(spritesheet.region(Rectangle {
                    w: SHIP_W,
                    h: SHIP_H,
                    x: SHIP_W * x as f64,
                    y: SHIP_H * y as f64,
                }).unwrap());
            }
        }

        //let (w, h) = sprite.size();

        ShipView {
            player: Ship {
                rect: Rectangle {
                    x: 64.0,
                    y: 64.0,
                    w: SHIP_W,
                    h: SHIP_H,
                },
                sprites: sprites,
                current: ShipFrame::MidNorm,
            },
            bg_back: Background {
                pos: 0.0,
                vel: 20.0,
                sprite: Sprite::load(&mut phi.renderer, "assets/starBG.png").unwrap(),
            },
            bg_middle: Background {
                pos: 0.0,
                vel: 40.0,
                sprite: Sprite::load(&mut phi.renderer, "assets/starMG.png").unwrap(),
            },
            bg_front: Background {
                pos: 0.0,
                vel: 80.0,
                sprite: Sprite::load(&mut phi.renderer, "assets/starFG.png").unwrap(),
            },
        }
    }
}

impl View for ShipView {

    fn render(&mut self, phi: &mut Phi, elapsed: f64) -> ViewAction {

        let events = &phi.events;

        if events.now.quit || events.now.key_escape == Some(true) {

            return ViewAction::Quit;
        }

        // Move the player's ship

        let diagonal =  (events.key_up ^ events.key_down)
                     && (events.key_left ^ events.key_right);

        let moved = if diagonal { 1.0 / 2.0f64.sqrt() }
                    else { 1.0 }
                    * PLAYER_SPEED * elapsed;

        let dx = match (events.key_left, events.key_right) {
            (true, true) | (false, false) => 0.0,
            (true, false) => -moved,
            (false, true) =>  moved,
        };

        let dy = match (events.key_up, events.key_down) {
            (true, true) | (false, false) => 0.0,
            (true, false) => -moved,
            (false, true) =>  moved,
        };

        // Create a bounding box - limit width to 70% of maximum
        let moveable_region = Rectangle {
            x: 0f64,
            y: 0f64,
            w: phi.output_size().0 * 0.70,
            h: phi.output_size().1,
        };

        self.player.rect.x += dx;
        self.player.rect.y += dy;
        self.player.rect =
            self.player.rect.move_inside(moveable_region).unwrap();

        // Select the appropriate sprite of the ship to show
        self.player.current =
            if      dx == 0.0 && dy < 0.0  { ShipFrame::UpNorm }
            else if dx > 0.0  && dy < 0.0  { ShipFrame::UpFast }
            else if dx < 0.0  && dy < 0.0  { ShipFrame::UpSlow }
            else if dx == 0.0 && dy == 0.0 { ShipFrame::MidNorm }
            else if dx > 0.0  && dy == 0.0 { ShipFrame::MidFast }
            else if dx < 0.0  && dy == 0.0 { ShipFrame::MidSlow }
            else if dx == 0.0 && dy > 0.0  { ShipFrame::DownNorm }
            else if dx > 0.0  && dy > 0.0  { ShipFrame::DownFast }
            else if dx < 0.0  && dy > 0.0  { ShipFrame::DownSlow }
            else { unreachable!() };

        // Clear the screen . . .

        phi.renderer.set_draw_color(Color::RGB(0, 0, 0));
        phi.renderer.clear();

        // Render the backgrounds . . .
        self.bg_back.render(&mut phi.renderer, elapsed);
        self.bg_middle.render(&mut phi.renderer, elapsed);

        // Render the bounding box (for debugging) . . .\
        if (DEBUG)
        {
            phi.renderer.set_draw_color(Color::RGB(200, 200, 50));
            phi.renderer.fill_rect(self.player.rect.to_sdl().unwrap());
        }

        // Render the ship . . .
        self.player.sprites[self.player.current as usize]
            .render(&mut phi.renderer, self.player.rect);

        // Render the foreground . . .
        self.bg_front.render(&mut phi.renderer, elapsed);

        ViewAction::None
    }

}

impl Background {
    fn render(&mut self, renderer: &mut Renderer, elapsed: f64) {
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
*/
