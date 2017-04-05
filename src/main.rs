extern crate minifb;
extern crate rand;
extern crate image;
extern crate fps_counter;
#[macro_use]
extern crate fixedstep;

mod app;
mod color;
mod map;
mod player;
mod point;

use app::{App, SCREEN_WIDTH, SCREEN_HEIGHT};
use map::Map;
use player::Player;

fn main() {
    let map = Map::new_random(24);
    let mut app = App::new(Player::new(22.5,
                                       12.5,
                                       -1.0,
                                       0.0,
                                       0.0,
                                       SCREEN_WIDTH as f64 / SCREEN_HEIGHT as f64 / 2.0),
                           &map);

    fixedstep_loop! {
        Update => {
            !app.update()
        },

        Render(_) => {
            app.draw();
        },
    }
}
