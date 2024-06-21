pub struct ConwayGameState {
    width: usize,
    height: usize,
    cells: Vec<bool>,
    prev_cells: Vec<bool>,
    generation: u64,
    symmetry: bool,
}

#[allow(dead_code)]
impl ConwayGameState {
    pub fn new(width: usize, height: usize, symmetry: bool) -> ConwayGameState {
        ConwayGameState {
            width,
            height,
            symmetry,
            cells: vec![false; width * height],
            prev_cells: vec![false; width * height],
            generation: 0,
        }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_cells(&self) -> &Vec<bool> {
        &self.cells
    }

    pub fn get_generation(&self) -> u64 {
        self.generation
    }

    pub fn get_cell(&self, x: usize, y: usize) -> bool {
        self.cells[y * self.width + x]
    }

    pub fn set_cell(&mut self, x: usize, y: usize, value: bool) {
        self.cells[y * self.width + x] = value;
    }

    pub fn step(&mut self) {
        assert!(self.cells.len() == self.width * self.height);
        assert!(self.prev_cells.len() == self.width * self.height);

        for y in 0..self.height {
            for x in 0..self.width {
                let count = self.count_neighors(x, y);
                let idx = y * self.width + x;

                if self.cells[idx] {
                    self.prev_cells[idx] = count == 2 || count == 3;
                } else {
                    self.prev_cells[idx] = count == 3;
                }
            }
        }
        std::mem::swap(&mut self.cells, &mut self.prev_cells);
    }

    fn count_neighors(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                if self.symmetry {
                    let nx = (x as isize + dx + self.width as isize) as usize % self.width;
                    let ny = (y as isize + dy + self.height as isize) as usize % self.height;
                    if self.cells[ny * self.width + nx] {
                        count += 1;
                    }
                } else {
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;
                    if nx >= 0 && nx < self.width as isize && ny >= 0 && ny < self.height as isize {
                        if self.cells[ny as usize * self.width + nx as usize] {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }
}
