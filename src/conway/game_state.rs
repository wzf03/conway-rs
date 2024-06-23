pub struct ConwayGameState {
    width: usize,
    height: usize,
    cells: Vec<bool>,
    prev_cells: Vec<bool>,
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

    pub fn set_cells(&mut self, cells: Vec<bool>) {
        assert!(cells.len() == self.width * self.height);
        self.cells = cells;
        self.prev_cells = vec![false; self.width * self.height];
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

    pub fn resize(&mut self, width: usize, height: usize) {
        // Resize the cells and prev_cells vectors
        self.cells = ConwayGameState::resize_cells(
            self.cells.clone(),
            (width, height),
            (self.width, self.height),
        );
        self.prev_cells = ConwayGameState::resize_cells(
            self.prev_cells.clone(),
            (width, height),
            (self.width, self.height),
        );
        self.width = width;
        self.height = height;
    }

    pub fn clear(&mut self) {
        self.cells.fill(false);
        self.prev_cells.fill(false);
    }

    fn resize_cells(
        cells: Vec<bool>,
        new_size: (usize, usize),
        old_size: (usize, usize),
    ) -> Vec<bool> {
        let mut new_cells = vec![false; new_size.0 * new_size.1];
        for y in 0..old_size.1.min(new_size.1) {
            for x in 0..old_size.0.min(new_size.0) {
                new_cells[y * new_size.0 + x] = cells[y * old_size.0 + x];
            }
        }
        new_cells
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

    pub fn get_newly_born_cells(&self) -> Vec<(usize, usize)> {
        let mut newly_born_cells = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;
                if self.cells[idx] && !self.prev_cells[idx] {
                    newly_born_cells.push((x, y));
                }
            }
        }
        newly_born_cells
    }

    pub fn get_newly_dead_cells(&self) -> Vec<(usize, usize)> {
        let mut newly_dead_cells = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;
                if !self.cells[idx] && self.prev_cells[idx] {
                    newly_dead_cells.push((x, y));
                }
            }
        }
        newly_dead_cells
    }

    pub fn get_retained_cells(&self) -> Vec<(usize, usize)> {
        let mut retained_cells = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;
                if self.cells[idx] && self.prev_cells[idx] {
                    retained_cells.push((x, y));
                }
            }
        }
        retained_cells
    }
}
