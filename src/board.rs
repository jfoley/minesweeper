extern crate rand;
use self::rand::Rng;

#[derive(PartialEq, Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Copy, Clone, Debug)]
pub struct Cell {
    pub mine: bool,
    pub flagged: bool,
    pub visible: bool,
    pub score: usize,
}

pub struct Board {
    cells: Vec<Vec<Cell>>
}

impl Board {
    pub fn new(size: usize, mines: Vec<Point>) -> Board {
        let mut cells = Vec::with_capacity(size);
        for x in 0..size {
            let mut cell_row = Vec::with_capacity(size);

            for y in 0..size {
                let mut cell = Cell{
                    mine: false,
                    flagged: false,
                    visible: false,
                    score: 0,
                };


                let mine = mines.iter().find(|m| *m == &Point{x: x, y: y});
                match mine {
                    Some(mine) => cell.mine = true,
                    None => cell.score = Board::score(&mines, size, x, y),
                }

                cell_row.push(cell);
            }

            cells.push(cell_row);
        }

        Board{cells: cells}
    }

    pub fn size(&self) -> usize {
        self.cells.len()
    }

    fn within_bounds(size: usize, x: isize, y: isize) -> bool {
        x >= 0 && x < size as isize && y >= 0 && y < size as isize
    }

    fn score(mines: &Vec<Point>, size: usize, x: usize, y: usize) -> usize {
        let mut score = 0;

        for n in Board::neighbors(size, x, y).iter() {
            if mines.iter().any(|m| m == n) {
                score += 1;
            }
        }

        score
    }

    fn neighbors(size: usize, x: usize, y: usize) -> Vec<Point> {
        let mut neighbors = Vec::new();

        for k in -1..2 {
            for i in -1..2 {
                if i == 0 && k == 0 {
                    continue;
                }

                let ix = x as isize + i;
                let iy = y as isize + k;

                if Board::within_bounds(size, ix, iy) {
                    neighbors.push(Point{x: ix as usize, y: iy as usize});
                }
            }
        }

        neighbors
    }

    pub fn uncover(&mut self, x: usize, y: usize) -> bool {
        self.cells[x][y].visible = true;
        if self.cells[x][y].score == 0 && !self.cells[x][y].mine {
            self.uncover_neighbors(x, y);
        }

        self.cells[x][y].mine
    }

    pub fn cell_at(&self, x: usize, y: usize) -> Cell {
        self.cells[x][y]
    }

    fn uncover_neighbors(&mut self, x: usize, y: usize) {
        for n in Board::neighbors(self.cells.len(), x, y).iter() {
            if self.show_cell(n.x, n.y) {
                self.uncover_neighbors(n.x, n.y);
            }
        }
    }

    fn show_cell(&mut self, x: usize, y: usize) -> bool {
        let mut cell = &mut self.cells[x][y];
        if !cell.visible && !cell.mine {
            cell.visible = true;
            true
        } else {
            false
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
fn uncovering_a_mine() {
    let mines = vec![
        Point{x: 0, y: 0},
    ];
    let mut board = Board::new(2, mines);

    let is_mine = board.uncover(0, 0);
    assert_eq!(is_mine, true);
    assert_eq!(board.cell_at(0, 0).visible, true);
    assert_eq!(board.cell_at(1, 0).visible, false);
    assert_eq!(board.cell_at(1, 1).visible, false);
    assert_eq!(board.cell_at(1, 1).visible, false)
}

#[test]
fn uncovering_tiles_with_scores() {
    let mines = vec![
        Point{x: 0, y: 0},
    ];

    let mut board = Board::new(2, mines);

    let is_mine = board.uncover(1, 0);
    assert_eq!(is_mine, false);
    assert_eq!(board.cell_at(0, 0).visible, false);
    assert_eq!(board.cell_at(1, 0).visible, true);
    assert_eq!(board.cell_at(0, 1).visible, false);
    assert_eq!(board.cell_at(1, 1).visible, false)
}

#[test]
fn recursively_uncovering_tiles() {
    let mines = vec![
        Point{x: 0, y: 0},
    ];

    let mut board = Board::new(3, mines);

    let is_mine = board.uncover(2, 2);
    assert_eq!(is_mine, false);
    assert_eq!(board.cell_at(0, 0).visible, false);
    assert_eq!(board.cell_at(0, 1).visible, true);
    assert_eq!(board.cell_at(0, 2).visible, true);
    assert_eq!(board.cell_at(1, 0).visible, true);
    assert_eq!(board.cell_at(1, 1).visible, true);
    assert_eq!(board.cell_at(1, 2).visible, true);
    assert_eq!(board.cell_at(2, 0).visible, true);
    assert_eq!(board.cell_at(2, 1).visible, true);
    assert_eq!(board.cell_at(2, 2).visible, true);
}

#[test]
fn scores() {
    let mines = vec![
        Point{x: 0, y: 0},
    ];

    let mut board = Board::new(2, mines);

    assert_eq!(board.cells[0][0].score, 0);
    assert_eq!(board.cells[0][1].score, 1);
    assert_eq!(board.cells[1][0].score, 1);
    assert_eq!(board.cells[1][1].score, 1);
}

#[test]
fn more_scores() {
    let mines = vec![
        Point{x: 0, y: 0},
        Point{x: 1, y: 1},
    ];

    let mut board = Board::new(3, mines);

    assert_eq!(board.cell_at(0, 0).score, 0);
    assert_eq!(board.cell_at(1, 0).score, 2);
    assert_eq!(board.cell_at(2, 0).score, 1);
    assert_eq!(board.cell_at(0, 1).score, 2);
    assert_eq!(board.cell_at(1, 1).score, 0);
    assert_eq!(board.cell_at(2, 1).score, 1);
    assert_eq!(board.cell_at(0, 2).score, 1);
    assert_eq!(board.cell_at(1, 2).score, 1);
    assert_eq!(board.cell_at(2, 2).score, 1)
}

#[test]
fn within_bounds_works() {
    assert_eq!(Board::within_bounds(3, -1, -1), false);
    assert_eq!(Board::within_bounds(3, 2, 2), true);
    assert_eq!(Board::within_bounds(3, 3, 3), false);
}

#[test]
fn calculate_score() {
    let mines = vec![
        Point{x: 1, y: 1},
    ];

    assert_eq!(Board::score(&mines, 2, 0, 0), 1);
    assert_eq!(Board::score(&mines, 2, 0, 1), 1);
    assert_eq!(Board::score(&mines, 2, 0, 2), 1);
    assert_eq!(Board::score(&mines, 2, 1, 0), 1);
    assert_eq!(Board::score(&mines, 2, 1, 2), 1);
    assert_eq!(Board::score(&mines, 2, 2, 0), 1);
    assert_eq!(Board::score(&mines, 2, 2, 1), 1);
    assert_eq!(Board::score(&mines, 2, 2, 2), 1);

    let mines = vec![
        Point{x: 0, y: 0},
        Point{x: 1, y: 1},
    ];

    assert_eq!(Board::score(&mines, 2, 1, 0), 2);
    assert_eq!(Board::score(&mines, 2, 0, 1), 2);
}

#[test]
fn neighbors_works() {
    let expected = vec![
        Point{x: 1, y: 0},
        Point{x: 0, y: 1},
        Point{x: 1, y: 1},
    ];

    let neighbors = Board::neighbors(2, 0, 0);
    assert_eq!(neighbors, expected);

    let expected = vec![
        Point{x: 0, y: 0},
        Point{x: 0, y: 1},
        Point{x: 1, y: 1},
    ];

    let neighbors = Board::neighbors(2, 1, 0);
    assert_eq!(neighbors, expected);

    let expected = vec![
        Point{x: 0, y: 0},
        Point{x: 2, y: 0},
        Point{x: 0, y: 1},
        Point{x: 1, y: 1},
        Point{x: 2, y: 1},
    ];

    let neighbors = Board::neighbors(3, 1, 0);
    assert_eq!(neighbors, expected);
}

#[test]
fn complicated_scores() {
    extern crate std;
    use board_printer::BoardWriter;

    use std::io::*;
    use board_printer::TestHelper;
let expected =
r#"┌──┬──┬──┬──┬──┬──┐
│  │1 │2 │3 │4 │5 │
│1 │1 │💣 │2 │2 │1 │
│2 │1 │3 │💣 │4 │💣 │
│3 │1 │3 │💣 │💣 │3 │
│4 │1 │💣 │3 │3 │💣 │
│5 │1 │1 │1 │1 │1 │
└──┴──┴──┴──┴──┴──┘
"#;

    let mines = vec![
        Point{x: 1, y: 0},
        Point{x: 2, y: 1},
        Point{x: 4, y: 1},
        Point{x: 2, y: 2},
        Point{x: 3, y: 2},
        Point{x: 1, y: 3},
        Point{x: 4, y: 3},
    ];

    let mut board = Board::new(5, mines);

    let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());

    {
        let mut writer = BoardWriter::new(&board, &mut cursor as &mut Write);
        writer.print_solution();
    }

    assert_eq!(cursor.into_string(), expected);
}
