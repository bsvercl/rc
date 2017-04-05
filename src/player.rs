use map::Map;

const PLAYER_MOVE_SPEED: f64 = 0.08;
const PLAYER_ROTATION_SPEED: f64 = 0.045;

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
        }
    }

    pub fn update<'a>(&mut self, map: &'a Map) {
        if self.moving_forward {
            self.move_in_direction(&map, true);
        } else if self.moving_backward {
            self.move_in_direction(&map, false);
        }

        if self.turning_left {
            self.turn_in_direction(true);
        } else if self.turning_right {
            self.turn_in_direction(false);
        }

        self.moving_forward = false;
        self.moving_backward = false;
        self.turning_left = false;
        self.turning_right = false;
    }

    fn move_in_direction(&mut self, map: &Map, forwards: bool) {
        let speed = if forwards {
            PLAYER_MOVE_SPEED
        } else {
            -PLAYER_MOVE_SPEED
        };

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