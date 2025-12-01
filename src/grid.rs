pub struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Vec<bool>>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let cells = vec![vec![false; width]; height];
        Grid {
            width,
            height,
            cells,
        }
    }

    pub fn set_cell(&mut self, x: usize, y: usize, value: bool) {
        if x < self.width && y < self.height {
            self.cells[y][x] = value;
        }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Option<bool> {
        if x < self.width && y < self.height {
            Some(self.cells[y][x])
        } else {
            None
        }
    }

    pub fn rotate_row(&mut self, y: usize, by: usize) {
        let by = by % self.width;
        let row = &mut self.cells[y];
        row.rotate_right(by);
    }

    pub fn rotate_column(&mut self, x: usize, by: usize) {
        let by = by % self.height;

        let mut column: Vec<bool> = self.cells.iter().map(|row| row[x]).collect();
        column.rotate_right(by);

        for (y, value) in column.into_iter().enumerate() {
            self.cells[y][x] = value;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn grid_instantiation() {
        let grid = Grid::new(3, 3);
        assert_eq!(grid.get_cell(0, 0), Some(false));
    }

    #[test]
    fn grid_set_cell() {
        let mut grid = Grid::new(3, 3);
        grid.set_cell(0, 0, true);
        assert_eq!(grid.get_cell(0, 0), Some(true));
    }

    #[test]
    fn grid_get_cell() {
        let mut grid = Grid::new(3, 3);
        assert_eq!(grid.get_cell(0, 1), Some(false));
        grid.set_cell(0, 1, true);
        assert_eq!(grid.get_cell(0, 1), Some(true));
    }

    #[test]
    fn grid_rotate_row() {
        let mut grid = Grid::new(3, 3);
        grid.set_cell(0, 0, true);
        grid.set_cell(1, 0, false);
        grid.set_cell(2, 0, true);

        grid.rotate_row(0, 1);

        assert_eq!(grid.get_cell(0, 0), Some(true));
        assert_eq!(grid.get_cell(1, 0), Some(true));
        assert_eq!(grid.get_cell(2, 0), Some(false));
    }

    #[test]
    fn grid_rotate_column() {
        let mut grid = Grid::new(3, 3);
        grid.set_cell(0, 0, true);
        grid.set_cell(0, 1, false);
        grid.set_cell(0, 2, true);

        grid.rotate_column(0, 1);

        assert_eq!(grid.get_cell(0, 0), Some(true));
        assert_eq!(grid.get_cell(0, 1), Some(true));
        assert_eq!(grid.get_cell(0, 2), Some(false));
    }
}
