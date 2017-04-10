use map::Map;

const PLAYER_MOVE_SPEED: f64 = 5.0;
const PLAYER_ROTATION_SPEED: f64 = 2.0;

pub struct Player {
    pub position_x: f64,
    pub position_y: f64,

    pub direction_x: f64,
    pub direction_y: f64,

    pub plane_x: f64,
    pub plane_y: f64,

    pub moving_forward: bool,
    pub moving_backward: bool,
    pub turning_left: bool,
    pub turning_right: bool,
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

            let move_step_x = self.direction_x * speed;
            let move_step_y = self.direction_y * speed;

            if map.get((self.position_x + move_step_x) as usize,
                       self.position_y as usize) == 0 {
                self.position_x += move_step_x;
            }

            if map.get(self.position_x as usize,
                       (self.position_y + move_step_y) as usize) == 0 {
                self.position_y += move_step_y;
            }
        }

        if self.turning_left || self.turning_right {
            let speed = if self.turning_left {
                PLAYER_ROTATION_SPEED * dt
            } else {
                -PLAYER_ROTATION_SPEED * dt
            };

            let c = speed.cos();
            let s = speed.sin();

            let old_direction_x = self.direction_x;
            self.direction_x = self.direction_x * c - self.direction_y * s;
            self.direction_y = old_direction_x * s + self.direction_y * c;
            let old_plane_x = self.plane_x;
            self.plane_x = self.plane_x * c - self.plane_y * s;
            self.plane_y = old_plane_x * s + self.plane_y * c;
        }
    }
}
