extern crate rand;
use self::rand::Rng;

mod board;
mod ui;
use ui::Ui;
use ui::MockUi;
use ui::RealUi;
use ui::Io;
use board::Board;
use std::str::FromStr;

mod board_printer;
use board_printer::BoardWriter;

use std::io;

struct App {
    board: Board,
    ui: RealUi,
}

impl App {
    fn new() -> App {
        let size = 10;
        let mut mines = Vec::new();

        let mut rng = rand::thread_rng();
        for i in 0..11 {
            mines.push(board::Point{x: rng.gen::<usize>() % size, y: rng.gen::<usize>() % size});
        }

        App{
            board: Board::new(size, mines),
            ui: RealUi::new()
        }
    }

    fn start(&mut self) {
        let mut stdout = std::io::stdout();

        loop {
            {
                let mut writer = BoardWriter::new(&self.board, &mut stdout);
                writer.print();
            }


            self.print_menu();
            let mut input = String::new();
            self.ui.ask(&mut input);

            match input.as_ref() {
                "1\n" => {
                    if self.flag() {
                        self.ui.say(&"WOO\n".to_string());
                        break;
                    }
                },

                "2\n" => {
                    if self.uncover() {
                        self.ui.say(&"sorry, you lost...\n".to_string());
                        {
                            let mut writer = BoardWriter::new(&self.board, &mut stdout);
                            writer.print_solution();
                        }
                        break;
                    }
                },
                "3\n" => break,
                _ => println!("unkown choice: {}", input)
            }
        }
    }

    fn flag(&mut self) -> bool {
        let (x, y) = self.get_coords();
        self.board.flag(x, y)
    }

    fn uncover(&mut self) -> bool {
        let (x, y) = self.get_coords();
        self.board.uncover(x, y)
    }

    fn print_menu(&mut self) {
        self.ui.say(&"\n".to_string());
        self.ui.say(&"1. flag\n".to_string());
        self.ui.say(&"2. uncover\n".to_string());
        self.ui.say(&"3. quit\n".to_string());
    }

    fn get_coords(&mut self) -> (usize, usize) {
        let mut x_input = String::new();
        self.ui.say(&"Enter x coordinate:\n".to_string());
        self.ui.ask(&mut x_input);

        let x = usize::from_str(x_input.trim()).unwrap();

        let mut y_input = String::new();
        self.ui.say(&"Enter y coordinate:\n".to_string());
        self.ui.ask(&mut y_input);
        let y = usize::from_str(y_input.trim()).unwrap();

        (x - 1, y - 1)
    }
}

fn main() {
    let mut app = App::new();
    app.start();
}

// #[test]
// fn it_can_quit() {
//     let mut ui = MockUi::new();
//     menu_expectation(&mut ui);
//     ui.expect(Io::Input("3\n".to_string()));
//
//     menu(&mut ui as &mut Ui);
//     assert!(true)
// }
//
// fn menu_expectation(ui: &mut MockUi) -> () {
//     ui
//         .expect(Io::Output("\n".to_string()))
//         .expect(Io::Output("1. flag\n".to_string()))
//         .expect(Io::Output("2. uncover\n".to_string()))
//         .expect(Io::Output("3. quit\n".to_string()));
//     ()
// }
//
// #[test]
// fn it_can_flag() {
//     let mut ui = MockUi::new();
//     menu_expectation(&mut ui);
//
//     ui
//         .expect(Io::Input("1\n".to_string()))
//         .expect(Io::Output("Enter x coordinate:\n".to_string()))
//         .expect(Io::Input("1\n".to_string()))
//         .expect(Io::Output("Enter y coordinate:\n".to_string()))
//         .expect(Io::Input("1\n".to_string()));
//
//     menu_expectation(&mut ui);
//     ui
//         .expect(Io::Input("3\n".to_string()));
//
//     menu(&mut ui as &mut Ui);
//     assert!(true)
// }
