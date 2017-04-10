use piston_window::*;
use cgmath::vec2;

use color;
use map::Map;
use player::Player;

pub const SCREEN_WIDTH: u32 = 640;
pub const SCREEN_HEIGHT: u32 = 480;
//const SCREEN_MIDDLE_X: u32 = SCREEN_WIDTH / 2;
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
        self.player.update(self.map, dt);
    }

    pub fn handle_key(&mut self, key: Key, pressed: bool) {
        match key {
            Key::W | Key::Up => self.player.moving_forward = pressed,
            Key::S | Key::Down => self.player.moving_backward = pressed,
            Key::A | Key::Left => self.player.turning_left = pressed,
            Key::D | Key::Right => self.player.turning_right = pressed,

            Key::LShift => self.player.running = pressed,

            _ => (),
        }
    }

    pub fn render(&mut self, c: &Context, g: &mut G2d) {
        clear(color::CORNFLOWER_BLUE, g);

        let screen_width_f64 = SCREEN_WIDTH as f64;
        let screen_height_f64 = SCREEN_HEIGHT as f64;
        let screen_middle_y_f64 = SCREEN_MIDDLE_Y as f64;

        rectangle(color::GRAY,
                  [0.0, 0.0, screen_width_f64, screen_middle_y_f64],
                  c.trans(0.0, screen_middle_y_f64).transform,
                  g);

        // draw walls
        for x in 0..SCREEN_WIDTH {
            let screen_coordinate: f64 = (x << 1) as f64 / screen_width_f64 - 1.0;
            let ray_position = self.player.position;
            let ray_direction = self.player.direction + self.player.plane * screen_coordinate;

            // coordinates on map from ray position
            let mut map_position = vec2(ray_position.x as isize, ray_position.y as isize);

            // length from one side to the other
            let ray_direction_x_squared = ray_direction.x * ray_direction.x;
            let ray_direction_y_squared = ray_direction.y * ray_direction.y;
            let delta = vec2((1.0 + ray_direction_y_squared / ray_direction_x_squared).sqrt(),
                             (1.0 + ray_direction_x_squared / ray_direction_y_squared).sqrt());

            // direction to step in x direction
            let (step_x, mut side_distance_x) = if ray_direction.x < 0.0 {
                (-1isize, (ray_position.x - map_position.x as f64) * delta.x)
            } else {
                (1isize, (map_position.x as f64 + 1.0 - ray_position.x) * delta.x)
            };

            // direction to step in y direction
            let (step_y, mut side_distance_y) = if ray_direction.y < 0.0 {
                (-1isize, (ray_position.y - map_position.y as f64) * delta.y)
            } else {
                (1isize, (map_position.y as f64 + 1.0 - ray_position.y) * delta.y)
            };

            let mut north_south_wall: bool = false;

            while self.map
                      .get(map_position.x as usize, map_position.y as usize) ==
                  0 {
                // jump to next square
                if side_distance_x < side_distance_y {
                    side_distance_x += delta.x;
                    map_position.x += step_x;
                    north_south_wall = false;
                } else {
                    side_distance_y += delta.y;
                    map_position.y += step_y;
                    north_south_wall = true;
                }
            }

            // distance to camera
            let wall_distance: f64 = if north_south_wall {
                ((map_position.y as f64 - ray_position.y + (1.0 - step_y as f64) / 2.0) /
                 ray_direction.y)
                        .abs()
            } else {
                ((map_position.x as f64 - ray_position.x + (1.0 - step_x as f64) / 2.0) /
                 ray_direction.x)
                        .abs()
            };

            // the height of the wall to be drawn
            let wall_height: f64 = (screen_height_f64 / wall_distance).abs();

            // find lowest and heighest pixels to be drawn
            let mut start: f64 = -wall_height / 2.0 + screen_middle_y_f64;
            if start < 0.0 {
                start = 0.0;
            }

            let mut end: f64 = wall_height / 2.0 + screen_middle_y_f64;
            if end > screen_height_f64 {
                end = screen_height_f64;
            }

            let mut color = match self.map
                      .get(map_position.x as usize, map_position.y as usize) {
                1 => color::RED,
                2 => color::GREEN,
                3 => color::BLUE,
                4 => color::ORANGE,
                5 => color::YELLOW,
                _ => color::WHITE,
            };

            let brightness = (wall_distance / 2.0) as f32;
            if brightness > 1.0 {
                for x in color.iter_mut().take(3) {
                    *x /= brightness;
                }
            }

            line(color,
                 1.0,
                 [0.0, start, 0.0, end],
                 c.trans(x as f64, 0.0).transform,
                 g);
        }
    }
}
