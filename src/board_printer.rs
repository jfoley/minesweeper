extern crate std;
use std::io::{BufStream, Cursor, Write, Read};

use board::{Board, Cell, Point};


pub struct BoardWriter<'a> {
    board: &'a Board,
    writer: &'a mut Write
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

static MINE: &'static str = "💣 ";
static EMPTY: &'static str = "  ";
static UNREVEALED: &'static str = ". ";
static FLAGGED: &'static str = "F ";

impl<'a> BoardWriter<'a> {
    pub fn new(board: &'a Board, writer: &'a mut Write) -> BoardWriter<'a> {
        BoardWriter{board: board, writer: writer}
    }

    pub fn print(&mut self) {
        self.print_header();

        for y in 0..self.board.size() {
            self.print_row(y, false);
        }

        self.print_footer();
    }

    pub fn print_solution(&mut self) {
        self.print_header();

        for i in 0..self.board.size() {
            self.print_row(i, true);
        }

        self.print_footer();
    }

    fn write(&mut self, string: String) {
        self.writer.write(&string.into_bytes()).unwrap();
    }

    fn print_header(&mut self) {
        self.write(TOP_LEFT.to_string());

        for i in 0..self.board.size() {
            self.write(TOP.to_string());
            self.write(TOP.to_string());
            self.write(TOP_MID.to_string());
        }

        self.write(TOP.to_string());
        self.write(TOP.to_string());
        self.write(TOP_RIGHT.to_string());
        self.write("\n".to_string());

        self.write(MID.to_string());
        self.write(EMPTY.to_string());

        for i in 0..self.board.size() {
            self.write(MID.to_string());
            self.print_label(i);
        }
        self.write(MID.to_string());
        self.write("\n".to_string());
    }

    fn print_label(&mut self, i: usize) {
        let label = i + 1;
        if label > 9 {
            self.write(format!("{}", label.to_string()));
        } else {
            self.write(format!("{} ", label.to_string()));
        }
    }

    fn print_footer(&mut self) {
        self.write(BOTTOM_LEFT.to_string());

        for i in 0..self.board.size() {
            self.write(BOTTOM.to_string());
            self.write(BOTTOM.to_string());
            self.write(BOTTOM_MID.to_string());
        }

        self.write(BOTTOM.to_string());
        self.write(BOTTOM.to_string());
        self.write(BOTTOM_RIGHT.to_string());
        self.write("\n".to_string());
    }

    pub fn print_row(&mut self, y: usize, solution: bool) {
        self.write(MID.to_string());
        self.print_label(y);

        for x in 0..self.board.size() {
            self.write(MID.to_string());

            let cell = self.board.cell_at(x, y);
            self.print_cell(cell, solution);

        }

        self.write(MID.to_string());
        self.write("\n".to_string());
    }

    fn print_cell(&mut self, cell: Cell, solution: bool) {
        if solution {
            if cell.mine {
                self.write(MINE.to_string());
            } else if cell.score > 0 {
                self.write(format!("{} ", cell.score.to_string()));
            } else {
                self.write(EMPTY.to_string());
            }
            return;
        }

        if cell.flagged {
            self.write(FLAGGED.to_string());
        } else if !cell.visible {
            self.write(UNREVEALED.to_string());
        } else if cell.mine {
            self.write(MINE.to_string());
        } else if cell.score > 0 {
            self.write(format!("{} ", cell.score.to_string()));
        } else {
            self.write(EMPTY.to_string());
        }
    }
}

pub trait TestHelper {
    fn into_string(self) -> String;
}

impl TestHelper for Cursor<Vec<u8>> {
    fn into_string(mut self) -> String {
        self.set_position(0);
        let mut buffer: Vec<u8> = Vec::new();
        self.read_to_end(&mut buffer).unwrap();
        String::from_utf8(buffer).unwrap()
    }
}

#[test]
fn writing() {
    let board = Board::new(5, vec![]);
    let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());

    {
        let mut writer = BoardWriter::new(&board, &mut cursor as &mut Write);
        writer.write("yo yo".to_string());
    }

    assert_eq!(cursor.into_string(), "yo yo");
}

#[test]
fn test_print_header() {
    let board = Board::new(5, vec![]);
    let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());

    {
        let mut writer = BoardWriter::new(&board, &mut cursor as &mut Write);
        writer.print_header();
    }

    assert_eq!(cursor.into_string(), "┌──┬──┬──┬──┬──┬──┐\n│  │1 │2 │3 │4 │5 │\n");
}

#[test]
fn test_print_footer() {
    let board = Board::new(5, vec![]);
    let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());

    {
        let mut writer = BoardWriter::new(&board, &mut cursor as &mut Write);
        writer.print_footer();
    }

    assert_eq!(cursor.into_string(), "└──┴──┴──┴──┴──┴──┘\n");
}

#[test]
fn test_print_row() {
    let mut board = Board::new(5, vec![
        Point{x: 0, y: 0}
    ]);

    board.uncover(1, 0);

    let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    {
        let mut writer = BoardWriter::new(&board, &mut cursor as &mut Write);
        writer.print_row(0, false);
    }

    assert_eq!(cursor.into_string(), "│1 │. │1 │. │. │. │\n");
}

#[test]
fn test_print_hidden_cell() {
    let cell = Cell{mine: false, flagged: false, visible: false, score: 8, coords: Point{x: 0, y: 0}};

    let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());

    {
        let board = Board::new(5, vec![]);
        let mut writer = BoardWriter::new(&board, &mut cursor as &mut Write);
        writer.print_cell(cell, false);
    }

    assert_eq!(cursor.into_string(), ". ");
}

#[test]
fn test_print_scored_cell() {
    let cell = Cell{mine: false, flagged: false, visible: true, score: 8, coords: Point{x: 0, y: 0}};

    let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());

    {
        let board = Board::new(5, vec![]);
        let mut writer = BoardWriter::new(&board, &mut cursor as &mut Write);
        writer.print_cell(cell, false);
    }

    assert_eq!(cursor.into_string(), "8 ");
}

#[test]
fn test_print_zero_cell() {
    let cell = Cell{mine: false, flagged: false, visible: true, score: 0, coords: Point{x: 0, y: 0}};

    let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());

    {
        let board = Board::new(5, vec![]);
        let mut writer = BoardWriter::new(&board, &mut cursor as &mut Write);
        writer.print_cell(cell, false);
    }

    assert_eq!(cursor.into_string(), "  ");
}

#[test]
fn test_print_mine_cell() {
    let cell = Cell{mine: true, flagged: false, visible: true, score: 0, coords: Point{x: 0, y: 0}};

    let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());

    {
        let board = Board::new(5, vec![]);
        let mut writer = BoardWriter::new(&board, &mut cursor as &mut Write);
        writer.print_cell(cell, false);
    }

    assert_eq!(cursor.into_string(), "💣 ");
}

#[test]
fn test_print_flagged_cell() {
    let cell = Cell{mine: true, flagged: true, visible: false, score: 0, coords: Point{x: 0, y: 0}};

    let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());

    {
        let board = Board::new(5, vec![]);
        let mut writer = BoardWriter::new(&board, &mut cursor as &mut Write);
        writer.print_cell(cell, false);
    }

    assert_eq!(cursor.into_string(), "F ");
}

#[test]
fn test_print() {
    let mut board = Board::new(5, vec![]);

    let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());

    {
        let mut writer = BoardWriter::new(&board, &mut cursor as &mut Write);
        writer.print();
    }
    let expected =

r#"┌──┬──┬──┬──┬──┬──┐
│  │1 │2 │3 │4 │5 │
│1 │. │. │. │. │. │
│2 │. │. │. │. │. │
│3 │. │. │. │. │. │
│4 │. │. │. │. │. │
│5 │. │. │. │. │. │
└──┴──┴──┴──┴──┴──┘
"#;

    assert_eq!(cursor.into_string(), expected);
}

#[test]
fn test_print_solution() {
    let mut board = Board::new(5, vec![
        Point{x: 0, y: 0},
        Point{x: 1, y: 1},
        Point{x: 2, y: 2},
        Point{x: 3, y: 3},
        Point{x: 4, y: 4},
    ]);

    let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());

    {
        let mut stdout = std::io::stdout();
        let mut writer = BoardWriter::new(&board, &mut cursor as &mut Write);
        writer.print_solution();
    }
    let expected =
r#"┌──┬──┬──┬──┬──┬──┐
│  │1 │2 │3 │4 │5 │
│1 │💣 │2 │1 │  │  │
│2 │2 │💣 │2 │1 │  │
│3 │1 │2 │💣 │2 │1 │
│4 │  │1 │2 │💣 │2 │
│5 │  │  │1 │2 │💣 │
└──┴──┴──┴──┴──┴──┘
"#;

    assert_eq!(cursor.into_string(), expected);
}
