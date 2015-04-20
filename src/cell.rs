use std::fmt::{Display, Formatter, Result};
static MINE: &'static str = "💣";

#[derive(Copy, Clone)]
enum Contents {
    Mine,
    Empty(usize)
}

fn score_string(score: usize) -> String {
    if score == 0 {
        "  ".to_string()
    }
    else {
        format!("{} ", score)
    }
}

impl Display for Contents {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        match *self {
            Contents::Mine => Display::fmt(MINE, fmt),
            Contents::Empty(score) => Display::fmt(&score_string(score), fmt),
        }
    }
}

#[derive(Copy, Clone)]
struct Cell {
    visible: bool,
    contents: Contents,
}

impl Display for Cell {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        if self.visible {
            Display::fmt(&self.contents, fmt)
        } else {
            Display::fmt(". ", fmt)
        }
    }
}

struct Writer {
    show_solution: bool
}

impl Writer {
    fn new(show_solution: bool) -> Writer {
        Writer{show_solution: show_solution}
    }

    fn print_cell(&self, cell: Cell) -> String {
        if self.show_solution {
            let mut visible_cell = cell.clone();
            visible_cell.visible = true;
            format!("{}", visible_cell)
        } else {
            format!("{}", cell)
        }
    }
}

#[test]
fn printing_hidden_cell() {
    let cell = Cell{visible: false, contents: Contents::Mine};
    let writer = Writer::new(false);

    let string = writer.print_cell(cell);

    assert_eq!(string, ". ");
}

#[test]
fn printing_revealed_cell() {
    let cell = Cell{visible: true, contents: Contents::Mine};
    let writer = Writer::new(false);

    let string = writer.print_cell(cell);

    assert_eq!(string, "💣");
}

#[test]
fn printing_hidden_cell_show_solution() {
    let cell = Cell{visible: false, contents: Contents::Mine};
    let writer = Writer::new(true);

    let string = writer.print_cell(cell);

    assert_eq!(string, "💣");
}

#[test]
fn printing_mines() {
    assert_eq!(format!("{}", Contents::Mine), "💣");
}

#[test]
fn printing_empty_cells() {
    assert_eq!(format!("{}", Contents::Empty(0)), "  ");
}

#[test]
fn printing_scored_cells() {
    assert_eq!(format!("{}", Contents::Empty(8)), "8 ");
}
