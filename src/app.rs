use minifb::{Key, Scale, Window, WindowOptions};

use color;
use map::Map;
use player::Player;

pub const SCREEN_WIDTH: usize = 640;
pub const SCREEN_HEIGHT: usize = 480;
const SCREEN_MIDDLE_X: usize = SCREEN_WIDTH / 2;
const SCREEN_MIDDLE_Y: usize = SCREEN_HEIGHT / 2;

pub struct App<'a> {
    window: Window,
    player: Player,
    map: &'a Map,

    buffer: Vec<u32>,
}

impl<'a> App<'a> {
    pub fn new(player: Player, map: &'a Map) -> Self {
        App {
            window: Window::new("rc",
                                SCREEN_WIDTH,
                                SCREEN_HEIGHT,
                                WindowOptions {
                                    scale: Scale::X2,
                                    ..WindowOptions::default()
                                })
                    .expect("failed to create window"),
            player: player,
            map: map,
            buffer: vec![0; SCREEN_WIDTH * SCREEN_HEIGHT],
        }
    }

    pub fn update(&mut self) -> bool {
        self.window
            .get_keys()
            .map(|keys| for key in keys {
                     self.handle_key(key);
                 });

        self.player.update(&self.map);

        self.window.is_open()
    }

    pub fn handle_key(&mut self, key: Key) {
        match key {
            Key::W => self.player.moving_forward = true,
            Key::S => self.player.moving_backward = true,
            Key::A => self.player.turning_left = true,
            Key::D => self.player.turning_right = true,

            _ => (),
        }
    }

    pub fn draw(&mut self) {
        // clear buffer
        self.draw_rectangle(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT, color::CORNFLOWER_BLUE);

        // draw the floor
        self.draw_rectangle(0,
                            SCREEN_MIDDLE_Y,
                            SCREEN_WIDTH,
                            SCREEN_MIDDLE_Y,
                            color::GRAY);

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
                2 => color::YELLOW,
                3 => color::BLUE,
                4 => color::GREEN,
                5 => color::ORANGE,
                _ => color::WHITE,
            };

            let alpha = (color >> 24) & 255;
            let mut red = (color >> 16) & 255;
            let mut green = (color >> 8) & 255;
            let mut blue = color & 255;

            let brightness = (wall_distance / 2.0) as u32;
            if brightness > 1 {
                red /= brightness;
                green /= brightness;
                blue /= brightness;
            }

            color = ((alpha & 255) << 24) | ((red & 255) << 16) | ((green & 255) << 8) |
                    (blue & 255);

            self.draw_line(x, start as usize, end as usize, color);
        }

        // draw crosshair
        let crosshair_size = 5;
        self.draw_rectangle(SCREEN_MIDDLE_X,
                            SCREEN_MIDDLE_Y - crosshair_size / 2,
                            1,
                            crosshair_size,
                            color::PINK);
        self.draw_rectangle(SCREEN_MIDDLE_X - crosshair_size / 2,
                            SCREEN_MIDDLE_Y,
                            crosshair_size,
                            1,
                            color::PINK);

        self.window.update_with_buffer(&self.buffer);
    }

    fn draw_line(&mut self, x: usize, y1: usize, y2: usize, color: u32) {
        for y in y1..y2 {
            self.buffer[x + SCREEN_WIDTH * y] = color;
        }
    }

    fn draw_rectangle(&mut self, x: usize, y: usize, w: usize, h: usize, color: u32) {
        for x in x..(x + w) {
            for y in y..(y + h) {
                self.buffer[x + SCREEN_WIDTH * y] = color;
            }
        }
    }
}
