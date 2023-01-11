use rand::random;

pub struct Board {
    board: Vec<Vec<bool>>,
    // x - y
    size: (usize, usize),
    pub iteration: u64,
}

impl Board {
    pub fn new(size_x: usize, size_y: usize, is_random: bool) -> Self {
        let mut board: Vec<Vec<bool>> = Vec::with_capacity(size_y);
        for y in 0..size_y {
            board.push(Vec::with_capacity(size_x));
            for _ in 0..size_x {
                let mut alive = false;

                if is_random {
                    alive = random();
                }

                board[y].push(alive);
            }
        }

        Board {
            board,
            size: (size_x, size_y),
            iteration: 0,
        }
    }

    pub fn get_alive_neighbors_of(&self, position: (usize, usize)) -> Option<u8> {
        let x = position.0 as i32;
        let y = position.1 as i32;

        if x > self.size.0 as i32 || y > self.size.1 as i32 || x < 0 || y < 0 {
            return None;
        }

        let mut alive = 0;

        for i in -1..=1 {
            for j in -1..=1 {
                let pos_x = x + j;
                let pos_y = y + i;

                // Skip out of bounds and self
                if pos_x < 0
                    || pos_y < 0
                    || (pos_x == x && pos_y == y)
                    || pos_x as usize >= self.size.0
                    || pos_y as usize >= self.size.1
                {
                    continue;
                }

                if self.board[pos_y as usize][pos_x as usize] {
                    alive += 1;
                }
            }
        }

        return Some(alive);
    }

    pub fn get_cells(&self) -> &Vec<Vec<bool>> {
        &self.board
    }

    pub fn next_generation(&mut self) {
        // let start = time::Instant::now();

        let mut new_board: Vec<Vec<bool>> = Vec::with_capacity(self.size.1);

        for y in 0..self.size.1 {
            let mut row: Vec<bool> = Vec::with_capacity(self.size.0);
            for x in 0..self.size.0 {
                let current_cell = &self.board[y][x];
                let neighbors_count = self
                    .get_alive_neighbors_of((x, y))
                    .expect("Failed to get value");

                let alive = *current_cell;

                if (!alive && neighbors_count == 3)
                    || (alive && (neighbors_count == 3 || neighbors_count == 2))
                {
                    row.push(true);
                } else {
                    row.push(false);
                }
            }
            new_board.push(row);
        }

        self.board = new_board;
        // println!("next_generation: {:?}", time::Instant::elapsed(&start));
    }
}
