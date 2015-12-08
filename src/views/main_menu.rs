// views/main_menu.rs

use ::phi::data::Rectangle;
use ::phi::gfx::Sprite;
use ::phi::{Phi, View, ViewAction};
use ::sdl2::pixels::Color;

pub struct MainMenuView {
    actions: Vec<Action>,
    selected: i8,
}

impl MainMenuView {
    pub fn new(phi: &mut Phi) -> MainMenuView {
        MainMenuView {
            actions: vec![
                Action::new(phi, "New Game", Box::new(|phi| {
                    ViewAction::ChangeView(Box::new(::views::game::ShipView::new(phi)))
                })),
                Action::new(phi, "Quit", Box::new(|_| {
                    ViewAction::Quit
                }))
            ],
            // Start at the top of the screen
            selected: 0,
        }
    }
}

impl View for MainMenuView {
    fn render(&mut self, phi: &mut Phi, elapsed: f64) -> ViewAction {
        if phi.events.now.quit || phi.events.now.key_escape == Some(true) {
            return ViewAction::Quit;
        }

        // Exceute the currently selected option
        if phi.events.now.key_space == Some(true) {
            // Use Rust functor syntax
            return (self.actions[self.selected as usize].func)(phi);
        }

        // Change the selected action using the keyboard
        if phi.events.now.key_up == Some(true) {
            self.selected -= 1;
            // Wrap around
            if self.selected < 0 {
                self.selected = self.actions.len() as i8 - 1;
            }
        }
        if phi.events.now.key_down == Some(true) {
            self.selected += 1;
            // Wrap around
            if self.selected >= self.actions.len() as i8 {
                self.selected = 0;
            }
        }

        // Clear the screen . . .
        phi.renderer.set_draw_color(Color::RGB(0, 0, 0));
        phi.renderer.clear();

        // Render the labels in the menu
        let (win_w, win_h) = phi.output_size();

        for (i, action) in self.actions.iter().enumerate() {
            if self.selected as usize == i {
                let (w, h) = action.hover_sprite.size();
                //phi.renderer.copy_sprite(&action.idle_sprite, Rectangle {
                action.hover_sprite.render(&mut phi.renderer, Rectangle {
                    x: (win_w - w) / 2.0,
                    y: 32.0 + 48.0 * i as f64,
                    w: w,
                    h: h,
                });
            }
            else {
                let (w, h) = action.idle_sprite.size();
                //phi.renderer.copy_sprite(&action.idle_sprite, Rectangle {
                action.idle_sprite.render(&mut phi.renderer, Rectangle {
                    x: (win_w - w) / 2.0,
                    y: 32.0 + 48.0 * i as f64,
                    w: w,
                    h: h,
                });
            }
        }

        ViewAction::None
    }
}

struct Action {
    // The function that should be executed if that action if chosen
    func: Box<Fn(&mut Phi) -> ViewAction>,

    // The sprite that is rendered when the user does not focus on this actions label.
    idle_sprite: Sprite,

    // The sprite that is rendered when the user focuses a label with the directional keys.
    hover_sprite: Sprite,
}

impl Action {
    fn new (phi: &mut Phi, label: &'static str, func: Box<Fn(&mut Phi) -> ViewAction>) -> Action {
        Action {
            func: func,
            idle_sprite: phi.ttf_str_sprite(label, "assets/belligerent.ttf", 32, Color::RGB(220, 220, 220)).unwrap(),
            hover_sprite: phi.ttf_str_sprite(label, "assets/belligerent.ttf", 32, Color::RGB(255, 255, 255)).unwrap(),
        }
    }
}
