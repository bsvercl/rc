extern crate minifb;
extern crate rand;
extern crate image;
extern crate fps_counter;
#[macro_use]
extern crate fixedstep;

use minifb::{Key, Scale, Window, WindowOptions};
use rand::{thread_rng, Rng};
use fps_counter::FPSCounter;

mod color {
    pub const RED: u32 = 0x00fa0a00;
    pub const YELLOW: u32 = 0x00f5fa00;
    pub const BLUE: u32 = 0x00000af0;
    pub const GREEN: u32 = 0x0000fa00;
    pub const ORANGE: u32 = 0x00ffa500;
    pub const WHITE: u32 = 0x00fffafa;

    pub const CORNFLOWER_BLUE: u32 = 0x00afbeff;
    pub const GRAY: u32 = 0x00808080;
}

const SCREEN_WIDTH: usize = 640;
const SCREEN_HEIGHT: usize = 480;
const SCREEN_MIDDLE_Y: usize = SCREEN_HEIGHT / 2;

struct Map {
    data: Vec<usize>,
    size: usize,
}

impl Map {
    fn new(data: &[usize], size: usize) -> Self {
        Map {
            data: data.to_vec(),
            size: size,
        }
    }

    fn new_random(size: usize) -> Self {
        let mut data: Vec<usize> = vec![0; size * size];
        for i in &mut data {
            if thread_rng().gen_range(0, 10) == 0 {
                *i = thread_rng().gen_range(2, 6);
            }
        }

        for i in 0..size {
            data[i + size * 0] = 1;
            data[i + size * (size - 1)] = 1;
        }

        for i in 0..size {
            data[0 + size * i] = 1;
            data[(size - 1) + size * i] = 1;
        }

        Map {
            data: data,
            size: size,
        }
    }

    fn get(&self, x: usize, y: usize) -> usize {
        let x = if x >= self.size { self.size - 1 } else { x };
        let y = if y >= self.size { self.size - 1 } else { y };
        self.data[x + self.size * y]
    }
}

// x + width * y
fn draw_line(buffer: &mut [u32], x: usize, y1: usize, y2: usize, color: u32) {
    for y in y1..y2 {
        buffer[x + SCREEN_WIDTH * y] = color;
    }
}

fn draw_rectangle(buffer: &mut [u32], x: usize, y: usize, w: usize, h: usize, color: u32) {
    for x in x..(x + w) {
        for y in y..(y + h) {
            buffer[x + SCREEN_WIDTH * y] = color;
        }
    }
}

const PLAYER_MOVE_SPEED: f64 = 0.08;
const PLAYER_ROTATION_SPEED: f64 = 0.045;

struct Player<'a> {
    position_x: f64,
    position_y: f64,

    direction_x: f64,
    direction_y: f64,

    plane_x: f64,
    plane_y: f64,

    moving_forward: bool,
    moving_backward: bool,

    turning_left: bool,
    turning_right: bool,

    map: &'a Map,
}

impl<'a> Player<'a> {
    fn new(position_x: f64,
           position_y: f64,

           direction_x: f64,
           direction_y: f64,

           plane_x: f64,
           plane_y: f64,

           map: &'a Map)
           -> Self {
        Player {
            position_x: position_x,
            position_y: position_y,

            direction_x: direction_x,
            direction_y: direction_y,

            plane_x: plane_x,
            plane_y: plane_y,

            moving_forward: false,
            moving_backward: false,

            turning_left: false,
            turning_right: false,

            map: map,
        }
    }

    fn update(&mut self) {
        if self.moving_forward {
            self.move_in_direction(true);
        } else if self.moving_backward {
            self.move_in_direction(false);
        }

        if self.turning_left {
            self.turn_in_direction(true);
        } else if self.turning_right {
            self.turn_in_direction(false);
        }
    }

    fn move_in_direction(&mut self, forwards: bool) {
        let speed = if forwards {
            PLAYER_MOVE_SPEED
        } else {
            -PLAYER_MOVE_SPEED
        };

        let move_step_x = self.direction_x * speed;
        let move_step_y = self.direction_y * speed;

        if self.map
               .get((self.position_x + move_step_x) as usize,
                    self.position_y as usize) == 0 {
            self.position_x += move_step_x;
        }

        if self.map
               .get(self.position_x as usize,
                    (self.position_y + move_step_y) as usize) == 0 {
            self.position_y += move_step_y;
        }
    }

    fn turn_in_direction(&mut self, left: bool) {
        let speed = if left {
            PLAYER_ROTATION_SPEED
        } else {
            -PLAYER_ROTATION_SPEED
        };

        let old_direction_x = self.direction_x;
        self.direction_x = self.direction_x * speed.cos() - self.direction_y * speed.sin();
        self.direction_y = old_direction_x * speed.sin() + self.direction_y * speed.cos();
        let old_plane_x = self.plane_x;
        self.plane_x = self.plane_x * speed.cos() - self.plane_y * speed.sin();
        self.plane_y = old_plane_x * speed.sin() + self.plane_y * speed.cos();
    }
}


fn handle_key(player: &mut Player, key: Key) {
    match key {
        Key::W => player.moving_forward = true,
        Key::S => player.moving_backward = true,
        Key::A => player.turning_left = true,
        Key::D => player.turning_right = true,

        _ => (),
    }
}

fn update(player: &mut Player) {
    player.update();

    player.moving_forward = false;
    player.moving_backward = false;
    player.turning_left = false;
    player.turning_right = false;
}

fn draw(buffer: &mut [u32], player: &mut Player, map: &Map) {
    // clear buffer
    draw_rectangle(buffer,
                   0,
                   0,
                   SCREEN_WIDTH,
                   SCREEN_HEIGHT,
                   color::CORNFLOWER_BLUE);

    // draw the floor
    draw_rectangle(buffer,
                   0,
                   SCREEN_MIDDLE_Y,
                   SCREEN_WIDTH,
                   SCREEN_MIDDLE_Y,
                   color::GRAY);

    // draw walls
    for x in 0..SCREEN_WIDTH {
        let screen_coordinate: f64 = (x << 1) as f64 / SCREEN_WIDTH as f64 - 1.0;
        let (ray_position_x, ray_position_y) = (player.position_x, player.position_y);
        let (ray_direction_x, ray_direction_y) =
            (player.direction_x + player.plane_x * screen_coordinate,
             player.direction_y + player.plane_y * screen_coordinate);

        let (mut map_x, mut map_y) = (ray_position_x as isize, ray_position_y as isize);

        let ray_direction_x_squared = ray_direction_x * ray_direction_x;
        let ray_direction_y_squared = ray_direction_y * ray_direction_y;

        let delta_x = (1.0 + ray_direction_y_squared / ray_direction_x_squared).sqrt();
        let delta_y = (1.0 + ray_direction_x_squared / ray_direction_y_squared).sqrt();

        let (step_x, mut side_distance_x) = if ray_direction_x < 0.0 {
            (-1, (ray_position_x - map_x as f64) * delta_x)
        } else {
            (1, (map_x as f64 + 1.0 - ray_position_x) * delta_x)
        };

        let (step_y, mut side_distance_y) = if ray_direction_y < 0.0 {
            (-1, (ray_position_y - map_y as f64) * delta_y)
        } else {
            (1, (map_y as f64 + 1.0 - ray_position_y) * delta_y)
        };

        let mut north_south_wall: bool = false;

        while map.get(map_x as usize, map_y as usize) == 0 {
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

        let wall_distance: f64 = if north_south_wall {
            ((map_y as f64 - ray_position_y + (1.0 - step_y as f64) / 2.0) / ray_direction_y).abs()
        } else {
            ((map_x as f64 - ray_position_x + (1.0 - step_x as f64) / 2.0) / ray_direction_x).abs()
        };

        let wall_height: isize = ((SCREEN_HEIGHT as f64 / wall_distance) as isize).abs();

        let mut start: isize = -wall_height / 2 + SCREEN_MIDDLE_Y as isize;
        if start < 0 {
            start = 0;
        }

        let mut end: isize = wall_height / 2 + SCREEN_MIDDLE_Y as isize;
        if end > SCREEN_HEIGHT as isize {
            end = SCREEN_HEIGHT as isize;
        }

        let mut color = match map.get(map_x as usize, map_y as usize) {
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

        color = ((alpha & 255) << 24) | ((red & 255) << 16) | ((green & 255) << 8) | (blue & 255);

        draw_line(buffer, x, start as usize, end as usize, color);
    }
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; SCREEN_WIDTH * SCREEN_HEIGHT];

    let mut window = Window::new("",
                                 SCREEN_WIDTH,
                                 SCREEN_HEIGHT,
                                 WindowOptions {
                                     scale: Scale::X2,
                                     ..WindowOptions::default()
                                 })
            .expect("failed  to create window");

    let map = Map::new_random(500);

    let mut player = Player::new(22.5,
                                 12.5,
                                 -1.0,
                                 0.0,
                                 0.0,
                                 SCREEN_WIDTH as f64 / SCREEN_HEIGHT as f64 / 2.0,
                                 &map);

    let mut counter = FPSCounter::new();

    fixedstep_loop! {
        Update => {
            window
                .get_keys()
                .map(|keys| for key in keys {
                         handle_key(&mut player, key);
                     });
            update(&mut player);

            !window.is_open()
        },

        Render(_) => {
            draw(&mut buffer, &mut player, &map);
            window.update_with_buffer(&buffer);
            window.set_title(format!("fps: {}", counter.tick()).as_str());
        },
    }
}
