use rand::{thread_rng, Rng};

pub struct Map {
    data: Vec<usize>,
    size: usize,
}

impl Map {
    pub fn random(size: usize) -> Self {
        let mut data: Vec<usize> = vec![0; size * size];
        // 1 in 10 chance to make a wall
        for i in &mut data {
            if thread_rng().gen_range(0, 10) == 0 {
                // random color
                *i = thread_rng().gen_range(2, 6);
            }
        }

        // borders around map
        for i in 0..size {
            data[i + size * 0] = 1;
            data[i + size * (size - 1)] = 1;
        }
        for i in 0..size {
            data[size * i] = 1;
            data[(size - 1) + size * i] = 1;
        }

        Map {
            data: data,
            size: size,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> usize {
        let x = if x >= self.size { self.size - 1 } else { x };
        let y = if y >= self.size { self.size - 1 } else { y };
        self.data[x + self.size * y]
    }
}
