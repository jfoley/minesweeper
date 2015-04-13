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
}

#[test]
fn creating_a_board() {
    let mines = vec![Point{x: 1, y: 1}];
    let board = Board::new(mines);

    assert_eq!(board.points[1][1].visible, false);
    assert_eq!(board.points[1][1].mine, true);
}
