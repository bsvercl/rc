extern crate piston_window;
extern crate sdl2_window;
extern crate rand;
extern crate image as im;
extern crate fps_counter;
extern crate cgmath;

mod app;
mod color;
mod map;
mod player;

use piston_window::*;
use sdl2_window::Sdl2Window;
use fps_counter::FPSCounter;

use app::App;
use map::Map;
use player::Player;

fn main() {
    let map = Map::new_random(500);
    let mut app = App::new(Player::new(22.5, 12.5, -1.0, 0.0, 0.0, 0.66), &map);

    let mut window: PistonWindow<Sdl2Window> = WindowSettings::new("", [640, 480]).build().unwrap();
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
            Input::Resize(width, height) => app.handle_resize(width, height),
            _ => {}
        }
    }
}
