extern crate rand;
use self::rand::Rng;

#[derive(PartialEq, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn left(self) -> Point {
        Point{x: self.x - 1, y: self.y }
    }

    fn top_left(self) -> Point {
        Point{x: self.x - 1, y: self.y - 1 }
    }

    fn top_right(self) -> Point {
        Point{x: self.x + 1, y: self.y - 1 }
    }

    fn top(self) -> Point {
        Point{x: self.x , y: self.y - 1 }
    }

    fn right(self) -> Point {
        Point{x: self.x + 1, y: self.y }
    }

    fn bottom_right(self) -> Point {
        Point{x: self.x + 1, y: self.y  + 1 }
    }

    fn bottom(self) -> Point {
        Point{x: self.x, y: self.y + 1 }
    }

    fn bottom_left(self) -> Point {
        Point{x: self.x - 1, y: self.y + 1 }
    }
}

#[derive(Copy, Clone)]
pub struct Cell {
    mine: bool,
    visible: bool,
}

impl Cell {
    fn make_visible(&mut self) {
        self.visible = true
    }
}

pub struct Board {
    cells: Vec<Vec<Cell>>
}

impl Board {
    fn new(size: usize, mines: Vec<Point>) -> Board {
        let mut rows = Vec::with_capacity(size);
        for y in 0..size {
            let mut cells = Vec::with_capacity(size);

            for x in 0..size {
                cells.push(Cell{mine: false, visible: false});
            }

            rows.push(cells);
        }

        for mine in mines.iter() {
            rows[mine.x][mine.y].mine = true;
        }

        Board{cells: rows}
    }

    fn uncover(&mut self, x: usize, y: usize) -> bool {
        self.cells[x][y].visible = true;
        self.uncover_neighbors(x, y);

        self.cells[x][y].mine
    }

    fn cell_at(&self, x: usize, y: usize) -> Cell {
        self.cells[x][y]
    }

    fn uncover_neighbors(&mut self, x: usize, y: usize) {
        for i in -1..2 {
            for k in -1..2 {
                if !(i == 0 && k == 0) {
                    self.show(x as isize + i, y as isize + k)
                }
            }
        }
    }

    fn show(&mut self, x: isize, y: isize) {
        if !self.within_bounds(x, y) {
            return;
        }

        let mut check_neighbors = false;
        {
            let mut cell = &mut self.cells[x as usize][y as usize];
            if !cell.visible && !cell.mine {
                // cell.make_visible();
                cell.visible = true;
                // println!("visible: {}", self.cell_at(x as usize, y as usize).visible)
                check_neighbors = true;
            }
        }

        if check_neighbors {
            self.uncover_neighbors(x as usize, y as usize);
        }
    }

    fn within_bounds(&self, x: isize, y: isize) -> bool {
        let max = self.cells.len() as isize;
        x >= 0 && x < max && y >= 0 && y < max
    }
}

#[test]
fn creating_a_board() {
    let mines = vec![Point{x: 0, y: 0}];
    let board = Board::new(1, mines);

    assert_eq!(board.cell_at(0, 0).visible, false);
    assert_eq!(board.cell_at(0, 0).mine, true);
}

#[test]
fn uncovering_tiles() {
    let mines = vec![
        Point{x: 0, y: 0},
        Point{x: 1, y: 1}
    ];
    let mut board = Board::new(2, mines);

    let is_mine = board.uncover(1, 0);
    assert_eq!(is_mine, false);
    assert_eq!(board.cell_at(0, 0).visible, false);
    assert_eq!(board.cell_at(1, 0).visible, true);
    assert_eq!(board.cell_at(1, 1).visible, false);
    assert_eq!(board.cell_at(1, 1).visible, false)
}

#[test]
fn recursively_uncovering_tiles() {
    let mines = vec![
        Point{x: 0, y: 0},
    ];

    let mut board = Board::new(2, mines);

    let is_mine = board.uncover(1, 0);
    assert_eq!(is_mine, false);
    assert_eq!(board.cell_at(0, 0).visible, false);
    assert_eq!(board.cell_at(1, 0).visible, true);
    assert_eq!(board.cell_at(1, 1).visible, true);
    assert_eq!(board.cell_at(1, 1).visible, true)
}
