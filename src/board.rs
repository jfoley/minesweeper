extern crate rand;
use self::rand::Rng;

struct Point {
    x: isize,
    y: isize
}

pub struct Board {
    size: isize,
    mines: Vec<Point>
}


static TOP_LEFT: &'static str = "┌";
static TOP_RIGHT: &'static str = "┐";
static TOP: &'static str = "─";
static TOP_MID: &'static str = "┬";
static MID: &'static str = "│";
static BOTTOM: &'static str = "─";
static BOTTOM_LEFT: &'static str = "└";
static BOTTOM_MID: &'static str = "┴";
static BOTTOM_RIGHT: &'static str = "┘";

impl Board {
    pub fn new(size: isize, mine_count: isize) -> Board {
        let mut mines = Vec::new();

        let mut rng = rand::thread_rng();

        for i in 0..mine_count {
            mines.push(Point{x: rng.gen::<isize>() % size, y: rng.gen::<isize>() % size});
        }

        Board{size: size, mines: mines}
    }

    pub fn print(&self, s: &mut String) -> () {
        self.print_header(s);

        for i in 0..self.size {
            self.print_row(s, i+1);
        }

        self.print_footer(s);
    }

    pub fn size(&self) -> & isize {
        &self.size
    }

    pub fn mine_size(&self) -> usize {
        self.mines.len()
    }

    fn print_header(&self, s: &mut String) -> () {
        s.push_str("\n");
        s.push_str(TOP_LEFT);

        for i in 0..self.size {
            s.push_str(TOP);
            s.push_str(TOP_MID);
        }

        s.push_str(TOP);
        s.push_str(TOP_RIGHT);
        s.push_str("\n");

        s.push_str(MID);
        s.push_str("*");

        for i in 0..self.size {
            s.push_str(MID);
            s.push_str(&(i + 1).to_string());
        }
        s.push_str(MID);
        s.push_str("\n");
    }

    fn print_row(&self, s: &mut String, row: isize) -> () {
        s.push_str(MID);
        s.push_str(&row.to_string());

        for i in 0..self.size {
            s.push_str(MID);
            s.push_str(".");
        }

        s.push_str(MID);
        s.push_str("\n");
    }

    fn print_footer(&self, s: &mut String) -> () {
        s.push_str(BOTTOM_LEFT);

        for i in 0..self.size {
            s.push_str(BOTTOM);
            s.push_str(BOTTOM_MID);
        }

        s.push_str(BOTTOM);
        s.push_str(BOTTOM_RIGHT);
        s.push_str("\n");
    }

}


#[test]
fn generate_board() {
    let mut board = Board::new(10, 10);

    assert_eq!(*board.size(), 10);
    assert_eq!(board.mine_size(), 10)
}

#[test]
fn print_board() {
    let mut board = Board::new(1, 1);

    let expected =
r#"
┌─┬─┐
│*│1│
│1│.│
└─┴─┘
"#;

    let mut board_string = String::new();
    board.print(&mut board_string);

    assert_eq!(board_string, expected)
}

#[test]
fn print_bigger_board() {
    let mut board = Board::new(5, 1);

    let expected =
r#"
┌─┬─┬─┬─┬─┬─┐
│*│1│2│3│4│5│
│1│.│.│.│.│.│
│2│.│.│.│.│.│
│3│.│.│.│.│.│
│4│.│.│.│.│.│
│5│.│.│.│.│.│
└─┴─┴─┴─┴─┴─┘
"#;

    let mut board_string = String::new();
    board.print(&mut board_string);

    println!("{}", board_string);
    println!("{}", expected);
    assert_eq!(board_string, expected)
}
