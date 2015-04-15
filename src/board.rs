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
                    None => {},
                }

                cell_row.push(cell);
            }

            cells.push(cell_row);
        }

        for y in 0..size {
            for x in 0..size {
                let cell = cells[x][y];

                if cell.mine {
                    continue;
                }

                let mut neighbor_mine_count = 0;
                for i in -1..2 {
                    for k in -1..2 {
                        let ux = x as isize + i;
                        let uy = y as isize + k;

                        if !Board::within_bounds(size, ux, uy) {
                            continue;
                        }

                        let neighbor = cells[ux as usize][uy as usize];
                        if neighbor.mine {
                            neighbor_mine_count += 1;
                        }
                    }
                }
                let mut cell = &mut cells[x][y];

                cell.score = neighbor_mine_count;

                println!("cell {} {} has score: {}", x, y, cell.score);
            }
        }

        Board{cells: cells}
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

                    if !Board::within_bounds(self.cells.len(), ux, uy) {
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

    fn within_bounds(size: usize, x: isize, y: isize) -> bool {
        x >= 0 && x < size as isize && y >= 0 && y < size as isize
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

    for y in 0..2 {
        for x in 0..2 {
            println!("{:?}", board.cells[x][y]);
        }
    }

    assert_eq!(board.cell_at(0, 0).score, 0);
    assert_eq!(board.cell_at(1, 0).score, 1);
    assert_eq!(board.cell_at(0, 1).score, 1);
    assert_eq!(board.cell_at(1, 1).score, 1)
}
