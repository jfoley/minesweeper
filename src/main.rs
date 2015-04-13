mod board;
mod ui;
use ui::Ui;
use ui::MockUi;
use ui::RealUi;
use ui::Io;
use board::Board;

extern crate ncurses;

fn main() {
    let board = Board::new(5, 1);

    ncurses::setlocale(ncurses::LcCategory::all, "");
    ncurses::initscr();
    ncurses::clear();
    ncurses::mvprintw(1, 4, "Hello 世界!");
    board.draw();
    ncurses::refresh();
    ncurses::getch();
    ncurses::nodelay(ncurses::stdscr, true); // don't block for input
    ncurses::noecho(); // don't echo input

    ncurses::endwin();
}

impl Board {
    fn draw(&self) {

    }
}
// fn main() {
//     let board = Board::new(5, 1);
//     let mut s = String::new();
//     board.print(&mut s);
//
//     println!("{}", s);
//
//     let mut ui = RealUi::new();
//     menu(&mut ui);
// }

fn print_menu(ui: &mut Ui) {
    ui.say(&"\n".to_string());
    ui.say(&"1. flag\n".to_string());
    ui.say(&"2. uncover\n".to_string());
    ui.say(&"3. quit\n".to_string());
}

fn menu(ui: &mut Ui) {
    loop {
        print_menu(ui);
        let mut input = String::new();
        ui.ask(&mut input);

        match input.as_ref() {
            "1\n" => get_coords(ui),
            "3\n" => break,
            _ => println!("unkown choice: {}", input)
        }
    }
}

fn get_coords(ui: &mut Ui) {
    let mut string = String::new();
    ui.say(&"Enter x coordinate:\n".to_string());
    ui.ask(&mut string);
    ui.say(&"Enter y coordinate:\n".to_string());
    ui.ask(&mut string)
}

#[test]
fn it_can_quit() {
    let mut ui = MockUi::new();
    menu_expectation(&mut ui);
    ui.expect(Io::Input("3\n".to_string()));

    menu(&mut ui as &mut Ui);
    assert!(true)
}

fn menu_expectation(ui: &mut MockUi) -> () {
    ui
        .expect(Io::Output("\n".to_string()))
        .expect(Io::Output("1. flag\n".to_string()))
        .expect(Io::Output("2. uncover\n".to_string()))
        .expect(Io::Output("3. quit\n".to_string()));
    ()
}

#[test]
fn it_can_flag() {
    let mut ui = MockUi::new();
    menu_expectation(&mut ui);

    ui
        .expect(Io::Input("1\n".to_string()))
        .expect(Io::Output("Enter x coordinate:\n".to_string()))
        .expect(Io::Input("1\n".to_string()))
        .expect(Io::Output("Enter y coordinate:\n".to_string()))
        .expect(Io::Input("1\n".to_string()));

    menu_expectation(&mut ui);
    ui
        .expect(Io::Input("3\n".to_string()));

    menu(&mut ui as &mut Ui);
    assert!(true)
}
