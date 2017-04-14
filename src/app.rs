use piston_window::*;
use piston_window::types::ColorComponent;
use cgmath::{Basis2, Rad, Rotation, Rotation2, vec2};

use color;
use map::Map;
use player::Player;

pub struct App<'a> {
    screen_width: u32,
    screen_height: u32,
    player: Player,
    map: &'a Map,
}

impl<'a> App<'a> {
    pub fn new(player: Player, map: &'a Map) -> Self {
        App {
            screen_width: 0,
            screen_height: 0,
            player: player,
            map: map,
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.player.update(self.map, dt);
    }

    pub fn handle_resize(&mut self, width: u32, height: u32) {
        self.screen_width = width;
        self.screen_height = height;
    }

    pub fn handle_key(&mut self, key: Key, pressed: bool) {
        match key {
            Key::W | Key::Up => self.player.moving_forward = pressed,
            Key::S | Key::Down => self.player.moving_backward = pressed,
            Key::A => self.player.moving_left = pressed,
            Key::D => self.player.moving_right = pressed,
            Key::LShift => self.player.running = pressed,

            _ => (),
        }
    }

    pub fn handle_mouse_relative(&mut self, x: f64, y: f64) {
        let speed = x * -1.0 * 0.01;

        let rot: Basis2<f64> = Rotation2::from_angle(Rad(speed));

        self.player.direction = rot.rotate_vector(self.player.direction);
        self.player.plane = rot.rotate_vector(self.player.plane);

        if (y > 0.0 && self.player.position.z > -750.0) ||
           (y < 0.0 && self.player.position.z < 750.0) {
            self.player.position.z += y * -1.5;
        }
    }

    pub fn render(&self, c: &Context, g: &mut G2d) {
        clear(color::CORNFLOWER_BLUE, g);

        let screen_width_f64 = self.screen_width as f64;
        let screen_height_f64 = self.screen_height as f64;
        let screen_middle_x_f64 = screen_width_f64 / 2.0;
        let screen_middle_y_f64 = screen_height_f64 / 2.0;

        if screen_middle_y_f64 + self.player.position.z < 0.0 {
            rectangle(color::GRAY,
                      [0.0, 0.0, screen_width_f64, screen_height_f64],
                      c.transform,
                      g);
        } else {
            rectangle(color::GRAY,
                      [0.0,
                       0.0,
                       screen_width_f64,
                       screen_middle_y_f64 - self.player.position.z],
                      c.trans(0.0, screen_middle_y_f64 + self.player.position.z)
                          .transform,
                      g);
        }

        // draw walls
        for x in 0..self.screen_width {
            let screen_coordinate: f64 = (x as f64 * 2.0) / screen_width_f64 - 1.0;
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

            let mut north_south_wall = false;

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
                    (map_position.y as f64 - ray_position.y + (1.0 - step_y as f64) / 2.0) /
                    ray_direction.y
                } else {
                    (map_position.x as f64 - ray_position.x + (1.0 - step_x as f64) / 2.0) /
                    ray_direction.x
                }
                .abs();

            // the height of the wall to be drawn
            let wall_height: f64 = (screen_height_f64 / wall_distance).abs();

            // find lowest and heighest pixels to be drawn
            let mut start: f64 = -wall_height / 2.0 + screen_middle_y_f64;
            if start < -self.player.position.z {
                start = -self.player.position.z;
            }

            let mut end: f64 = wall_height / 2.0 + screen_middle_y_f64;
            if end > screen_height_f64 - self.player.position.z {
                end = screen_height_f64 - self.player.position.z;
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

            if north_south_wall {
                for x in color.iter_mut().take(3) {
                    *x /= 2.0;
                }
            }

            let brightness = (wall_distance / 2.0) as ColorComponent;
            if brightness > 1.0 {
                for x in color.iter_mut().take(3) {
                    *x /= brightness;
                }
            }

            let mut y1 = start + self.player.position.z;
            if y1 < 0.0 {
                y1 = 0.0;
            }

            let mut y2 = end + self.player.position.z;
            if y2 < 0.0 {
                y2 = 0.0;
            }

            line(color,
                 1.0,
                 [0.0, y1, 0.0, y2],
                 c.trans(x as f64, 0.0).transform,
                 g);
        }

        // draw crosshair
        let crosshair_size = 2.0f64;
        rectangle(color::PINK,
                  [0.0, 0.0, crosshair_size, crosshair_size],
                  c.trans(screen_middle_x_f64 - crosshair_size,
                          screen_middle_y_f64 - crosshair_size)
                      .transform,
                  g);
    }
}
