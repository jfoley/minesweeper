use std::io::{BufStream, Cursor, Write, Read};
use std::ops::Deref;

use board::Board;

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

impl<'a> BoardWriter<'a> {
    pub fn new(board: &'a Board, writer: &'a mut Write) -> BoardWriter<'a> {
        BoardWriter{board: board, writer: writer}
    }

    pub fn write(&mut self, string: String) {
        self.writer.write(&string.into_bytes()).unwrap();
    }

    pub fn print_header(&mut self) -> () {
        self.writer.write(TOP_LEFT.as_bytes());

        for i in 0..self.board.size() {
            self.writer.write(TOP.as_bytes());
            self.writer.write(TOP_MID.as_bytes());
        }

        self.writer.write(TOP.as_bytes());
        self.writer.write(TOP_RIGHT.as_bytes());
        self.writer.write("\n".as_bytes());

        self.writer.write(MID.as_bytes());
        self.writer.write(" ".as_bytes());

        for i in 0..self.board.size() {
            self.writer.write(MID.as_bytes());
            self.writer.write(&(i + 1).to_string().as_bytes());
        }
        self.writer.write(MID.as_bytes());
        self.writer.write("\n".as_bytes());
    }
}

trait TestHelper {
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

    assert_eq!(cursor.into_string(), "┌─┬─┬─┬─┬─┬─┐\n│ │1│2│3│4│5│\n");
}
