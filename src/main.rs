extern crate piston_window;

extern crate rand;
extern crate image as im;
extern crate fps_counter;
extern crate cgmath;

mod app;
mod color;
mod map;
mod player;

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
    window.set_capture_cursor(true);
    let mut counter = FPSCounter::new();
    let mut cursor_captured = true;

    let mut glyphs = Glyphs::new("assets/fonts/InputMono-Regular.ttf", window.factory.clone())
        .unwrap();

    while let Some(e) = window.next() {
        match e {
            Input::Update(args) => app.update(args.dt),
            Input::Render(_) => {
                window
                    .draw_2d(&e, |c, g| {
                        app.render(&c, g);
                        text([1.0, 0.0, 0.0, 1.0],
                             11,
                             &format!("fps: {}", counter.tick()),
                             &mut glyphs,
                             c.trans(1.0, 12.0).transform,
                             g);
                    })
                    .unwrap();
            }
            Input::Press(Button::Keyboard(key)) => {
                app.handle_key(key, true);
                if key == Key::Escape {
                    cursor_captured = !cursor_captured;
                    window.set_capture_cursor(cursor_captured);
                }
            }
            Input::Release(Button::Keyboard(key)) => app.handle_key(key, false),
            Input::Move(Motion::MouseRelative(x, y)) => app.handle_mouse_relative(x, y),
            _ => {}
        }
    }

}
