use piston_window::*;
use piston_window::types::Color;

use color;
use map::Map;
use player::Player;

use im;

pub const SCREEN_WIDTH: u32 = 640;
pub const SCREEN_HEIGHT: u32 = 480;
const SCREEN_MIDDLE_X: u32 = SCREEN_WIDTH / 2;
const SCREEN_MIDDLE_Y: u32 = SCREEN_HEIGHT / 2;

pub struct App<'a> {
    player: Player,
    map: &'a Map,
}

impl<'a> App<'a> {
    pub fn new(player: Player, map: &'a Map) -> Self {
        App {
            player: player,
            map: map,
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.player.update(&self.map, dt);
    }

    pub fn handle_key(&mut self, key: Key, pressed: bool) {
        match key {
            Key::W => self.player.moving_forward = pressed,
            Key::S => self.player.moving_backward = pressed,
            Key::A => self.player.turning_left = pressed,
            Key::D => self.player.turning_right = pressed,

            _ => (),
        }
    }

    pub fn render(&mut self, c: &Context, g: &mut G2d) {
        clear(color::CORNFLOWER_BLUE, g);

        rectangle(color::GRAY,
                  [0.0, 0.0, SCREEN_WIDTH as f64, SCREEN_MIDDLE_Y as f64],
                  c.trans(0.0, SCREEN_MIDDLE_Y as f64).transform,
                  g);

        // draw walls
        for x in 0..SCREEN_WIDTH {
            let screen_coordinate: f64 = (x << 1) as f64 / SCREEN_WIDTH as f64 - 1.0;
            let (ray_position_x, ray_position_y) = (self.player.position_x, self.player.position_y);
            let (ray_direction_x, ray_direction_y) =
                (self.player.direction_x + self.player.plane_x * screen_coordinate,
                 self.player.direction_y + self.player.plane_y * screen_coordinate);

            // coordinates on map from ray position
            let (mut map_x, mut map_y) = (ray_position_x as isize, ray_position_y as isize);

            // length from one side to the other
            let ray_direction_x_squared = ray_direction_x * ray_direction_x;
            let ray_direction_y_squared = ray_direction_y * ray_direction_y;
            let delta_x = (1.0 + ray_direction_y_squared / ray_direction_x_squared).sqrt();
            let delta_y = (1.0 + ray_direction_x_squared / ray_direction_y_squared).sqrt();

            // direction to step in x direction
            let (step_x, mut side_distance_x) = if ray_direction_x < 0.0 {
                (-1, (ray_position_x - map_x as f64) * delta_x)
            } else {
                (1, (map_x as f64 + 1.0 - ray_position_x) * delta_x)
            };

            // direction to step in y direction
            let (step_y, mut side_distance_y) = if ray_direction_y < 0.0 {
                (-1, (ray_position_y - map_y as f64) * delta_y)
            } else {
                (1, (map_y as f64 + 1.0 - ray_position_y) * delta_y)
            };

            let mut north_south_wall: bool = false;

            while self.map.get(map_x as usize, map_y as usize) == 0 {
                // jump to next square
                if side_distance_x < side_distance_y {
                    side_distance_x += delta_x;
                    map_x += step_x;
                    north_south_wall = false;
                } else {
                    side_distance_y += delta_y;
                    map_y += step_y;
                    north_south_wall = true;
                }
            }

            // distance to camera
            let wall_distance: f64 = if north_south_wall {
                ((map_y as f64 - ray_position_y + (1.0 - step_y as f64) / 2.0) / ray_direction_y)
                    .abs()
            } else {
                ((map_x as f64 - ray_position_x + (1.0 - step_x as f64) / 2.0) / ray_direction_x)
                    .abs()
            };

            // the height of the wall to be drawn
            let wall_height: isize = ((SCREEN_HEIGHT as f64 / wall_distance) as isize).abs();

            // find lowest and heighest pixels to be drawn
            let mut start: isize = -wall_height / 2 + SCREEN_MIDDLE_Y as isize;
            if start < 0 {
                start = 0;
            }

            let mut end: isize = wall_height / 2 + SCREEN_MIDDLE_Y as isize;
            if end > SCREEN_HEIGHT as isize {
                end = SCREEN_HEIGHT as isize;
            }

            let mut color = match self.map.get(map_x as usize, map_y as usize) {
                1 => color::RED,
                2 => color::GREEN,
                3 => color::BLUE,
                4 => color::ORANGE,
                5 => color::YELLOW,
                _ => color::WHITE,
            };

            let brightness = (wall_distance / 2.0) as f32;
            if brightness > 1.0 {
                for x in 0..3 {
                    color[x] /= brightness;
                }
            }

            line(color,
                 1.0,
                 [x as f64, start as f64, x as f64, end as f64],
                 c.transform,
                 g);
        }
    }
}
