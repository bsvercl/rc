extern crate piston_window;
extern crate rand;
extern crate image as im;
extern crate fps_counter;

mod app;
mod color;
mod map;
mod player;
mod point;

use piston_window::*;
use fps_counter::FPSCounter;

use app::{App, SCREEN_WIDTH, SCREEN_HEIGHT};
use map::Map;
use player::Player;

use std::f64;

fn main() {
    let map = Map::new_random(500);
    // let fov: f64 = 90.0;
    // let mut app = App::new(Player::new(22.5, 12.5, -1.0, 0.0, 0.0, fov.to_radians()),
    //                        &map);

    let mut app = App::new(Player::new(22.5,
                                       12.5,
                                       -1.0,
                                       0.0,
                                       0.0,
                                       SCREEN_WIDTH as f64 / SCREEN_HEIGHT as f64 / 2.0),
                           &map);

    let mut window: PistonWindow = WindowSettings::new("", [SCREEN_WIDTH, SCREEN_HEIGHT])
        .build()
        .unwrap();
    let mut counter = FPSCounter::new();

    while let Some(e) = window.next() {
        match e {
            Input::Update(args) => app.update(args.dt),
            Input::Render(_) => {
                window.draw_2d(&e, |c, g| app.render(&c, g)).unwrap();
                window.set_title(format!("rc | fps: {}", counter.tick()));
            }
            Input::Press(Button::Keyboard(key)) => app.handle_key(key, true),
            Input::Release(Button::Keyboard(key)) => app.handle_key(key, false),
            _ => {}
        }
    }

}
