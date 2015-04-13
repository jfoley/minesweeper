extern crate rand;
use self::rand::Rng;

#[derive(PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone)]
pub struct Tile {
    mine: bool,
    visible: bool,
}

pub struct Board {
    points: [[Tile; 10]; 10]
}

impl Board {
    fn new(mines: Vec<Point>) -> Board {
        let mut board = Board{points: [[Tile{mine: false, visible: false} ;10]; 10]};

        for mine in mines.iter() {
            board.points[mine.x][mine.y].mine = true;
        }

        board
    }

    fn uncover(&mut self, point: Point) -> bool {
        self.points[point.x][point.y].visible = true;
        self.points[point.x][point.y].mine
    }
}

#[test]
fn creating_a_board() {
    let mines = vec![Point{x: 1, y: 1}];
    let board = Board::new(mines);

    assert_eq!(board.points[1][1].visible, false);
    assert_eq!(board.points[1][1].mine, true);
}

#[test]
fn uncovering_tiles() {
    let mines = vec![Point{x: 1, y: 1}];
    let mut board = Board::new(mines);

    let is_mine = board.uncover(Point{x: 2, y: 1});
    assert_eq!(is_mine, false);
    assert_eq!(board.points[2][1].visible, true)
}
