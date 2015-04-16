extern crate rand;
use self::rand::Rng;

#[derive(PartialEq, Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone, Debug)]
pub struct Cell {
    mine: bool,
    visible: bool,
    score: usize,
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
        let mut cells = Vec::with_capacity(size);
        for y in 0..size {
            let mut cell_row = Vec::with_capacity(size);

            for x in 0..size {
                let mut cell = Cell{
                    mine: false,
                    visible: false,
                    score: 0,
                };


                let mine = mines.iter().find(|m| *m == &Point{x: x, y: y});
                match mine {
                    Some(mine) => cell.mine = true,
                    None => cell.score = Board::score(&mines, cells.len(), x, y),
                }

                cell_row.push(cell);
            }

            cells.push(cell_row);
        }

        Board{cells: cells}
    }

    fn within_bounds(size: usize, x: isize, y: isize) -> bool {
        x >= 0 && x <= size as isize && y >= 0 && y <= size as isize
    }

    fn score(mines: &Vec<Point>, size: usize, x: usize, y: usize) -> usize {
        let mut score = 0;
        let mut neighbors = Vec::new();

        for k in -1..2 {
            for i in -1..2 {
                if i == 0 && k == 0 {
                    continue;
                }

                let ux = x as isize + i;
                let uy = y as isize + k;

                if Board::within_bounds(size, ux, uy) {
                    neighbors.push(Point{x: ux as usize, y: uy as usize});
                }
            }
        }

        for n in neighbors.iter() {
            if mines.iter().any(|m| m == n) {
                score += 1;
            }
        }

        score
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
                    let ux = x as isize + i;
                    let uy = y as isize + k;

                    if !Board::within_bounds(self.cells.len() - 1, ux, uy) {
                        continue;
                    }

                    self.show(ux as usize, uy as usize);
                }
            }
        }
    }

    fn show(&mut self, x: usize, y: usize) {
        if self.show_cell(x, y) {
            self.uncover_neighbors(x, y);
        }
    }

    fn show_cell(&mut self, x: usize, y: usize) -> bool {
        let mut cell = &mut self.cells[x][y];
        if !cell.visible && !cell.mine {
            cell.make_visible();
            true
        } else {
            false
        }
    }

    fn print(&self) {
        for y in 0..self.cells.len() {
            for x in 0..self.cells.len() {
                println!("({}, {}) {:?}", x, y, self.cells[x][y])
            }
        }
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
    assert_eq!(board.cell_at(0, 1).visible, true);
    assert_eq!(board.cell_at(1, 1).visible, true)
}

#[test]
fn scores() {
    let mines = vec![
        Point{x: 0, y: 0},
    ];

    let mut board = Board::new(2, mines);

    assert_eq!(board.cell_at(0, 0).score, 0);
    assert_eq!(board.cell_at(1, 0).score, 1);
    assert_eq!(board.cell_at(0, 1).score, 1);
    assert_eq!(board.cell_at(1, 1).score, 1)
}

#[test]
fn more_scores() {
    let mines = vec![
        Point{x: 0, y: 0},
        Point{x: 1, y: 1},
    ];

    let mut board = Board::new(3, mines);
    board.print();

    assert_eq!(board.cell_at(0, 0).score, 0);
    assert_eq!(board.cell_at(1, 0).score, 2);
    assert_eq!(board.cell_at(2, 0).score, 1);
    assert_eq!(board.cell_at(0, 1).score, 1);
    assert_eq!(board.cell_at(1, 1).score, 0);
    assert_eq!(board.cell_at(2, 1).score, 1);
    assert_eq!(board.cell_at(0, 2).score, 0);
    assert_eq!(board.cell_at(1, 2).score, 1);
    assert_eq!(board.cell_at(2, 2).score, 1)
}

#[test]
fn within_bounds_works() {
    assert_eq!(Board::within_bounds(3, -1, -1), false);
    assert_eq!(Board::within_bounds(3, 2, 2), true);
    // assert_eq!(Board::within_bounds(3, 3, 3), false);
}


#[test]
fn calculate_score() {
    let mines = vec![
        Point{x: 0, y: 0},
    ];

    assert_eq!(Board::score(&mines, 2, 1, 0), 1);
}
