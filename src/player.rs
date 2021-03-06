use map::Map;
use cgmath::{Vector2, Vector3, vec2, vec3};

const PLAYER_MOVE_SPEED: f64 = 5.0;
//const PLAYER_ROTATION_SPEED: f64 = 2.0;

pub struct Player {
    pub position: Vector3<f64>,
    pub direction: Vector2<f64>,
    pub plane: Vector2<f64>,

    pub moving_forward: bool,
    pub moving_backward: bool,
    pub moving_left: bool,
    pub moving_right: bool,
    pub running: bool,
}

impl Player {
    pub fn new(position_x: f64,
               position_y: f64,

               direction_x: f64,
               direction_y: f64,

               plane_x: f64,
               plane_y: f64)
               -> Self {
        Player {
            position: vec3(position_x, position_y, 0.0),
            direction: vec2(direction_x, direction_y),
            plane: vec2(plane_x, plane_y),

            moving_forward: false,
            moving_backward: false,
            moving_left: false,
            moving_right: false,
            running: false,
        }
    }

    pub fn update(&mut self, map: &Map, dt: f64) {
        if self.moving_forward || self.moving_backward {
            let speed = if self.moving_forward {
                PLAYER_MOVE_SPEED * dt
            } else {
                -PLAYER_MOVE_SPEED * dt
            };
            let speed = if self.running { speed * 2.0 } else { speed };

            let move_step = self.direction * speed;

            if map.get((self.position.x + move_step.x) as usize,
                       self.position.y as usize) == 0 {
                self.position.x += move_step.x;
            }

            if map.get(self.position.x as usize,
                       (self.position.y + move_step.y) as usize) == 0 {
                self.position.y += move_step.y;
            }
        }

        if self.moving_left || self.moving_right {
            let speed = if self.moving_left {
                PLAYER_MOVE_SPEED
            } else {
                -PLAYER_MOVE_SPEED
            } * dt;
            let speed = if self.running { speed * 2.0 } else { speed };

            let move_step = -self.plane * speed;

            if map.get((self.position.x + move_step.x) as usize,
                       (self.position.y + move_step.y) as usize) == 0 {
                self.position += move_step.extend(0.0);
            }
        }

        /*if self.turning_left || self.turning_right {
            let speed = if self.turning_left {
                PLAYER_ROTATION_SPEED * dt
            } else {
                -PLAYER_ROTATION_SPEED * dt
            };

            let rot: Basis2<f64> = Rotation2::from_angle(Rad(speed));

            self.direction = rot.rotate_vector(self.direction);
            self.plane = rot.rotate_vector(self.plane);
        }*/
    }
}
