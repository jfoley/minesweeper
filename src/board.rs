extern crate rand;
use self::rand::Rng;

#[derive(PartialEq, Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone)]
pub struct Cell {
    mine: bool,
    visible: bool,
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

    fn uncover(&mut self, point: Point) -> bool {
        self.cells[point.x][point.y].visible = true;
        self.cells[point.x][point.y].mine
    }

    fn cell_at(&self, point: Point) -> Cell {
        self.cells[point.x][point.y]
    }
}

#[test]
fn creating_a_board() {
    let mines = vec![Point{x: 0, y: 0}];
    let board = Board::new(1, mines);

    assert_eq!(board.cell_at(Point{x: 0, y: 0}).visible, false);
    assert_eq!(board.cell_at(Point{x: 0, y: 0}).mine, true);
}

#[test]
fn uncovering_tiles() {
    let mines = vec![
        Point{x: 0, y: 0},
        Point{x: 1, y: 1}
    ];
    let mut board = Board::new(2, mines);

    let is_mine = board.uncover(Point{x: 1, y: 0});
    assert_eq!(is_mine, false);
    assert_eq!(board.cell_at(Point{x: 0, y: 0}).visible, false);
    assert_eq!(board.cell_at(Point{x: 1, y: 0}).visible, true);
    assert_eq!(board.cell_at(Point{x: 1, y: 1}).visible, false);
    assert_eq!(board.cell_at(Point{x: 1, y: 1}).visible, false)
}
