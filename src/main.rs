mod board;
mod ui;
use ui::Ui;
use ui::MockUi;
use ui::Io;
use board::Board;

fn main() {
    let board = Board::new(5, 1);
    let mut s = String::new();
    board.print(&mut s);

    println!("{}", s)
}

fn print_menu(s: &mut String) {
    s.push_str("\n");
    s.push_str("1. flag\n");
    s.push_str("2. uncover\n");
    s.push_str("3. quit\n");
}

fn menu(ui: &mut Ui) {
    loop {
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
fn print_menu_works() {
    let mut s = String::new();
    print_menu(&mut s);

    let expected =
r#"
1. flag
2. uncover
3. quit
"#;
    assert_eq!(s, expected)
}

#[test]
fn it_can_quit() {
    let mut ui = MockUi::new();
    ui.expect(Io::Input("3\n".to_string()));

    menu(&mut ui as &mut Ui);
    assert!(true)
}

#[test]
fn it_can_flag() {
    let mut ui = MockUi::new();
    ui
        .expect(Io::Input("1\n".to_string()))
        .expect(Io::Output("Enter x coordinate:\n".to_string()))
        .expect(Io::Input("1\n".to_string()))
        .expect(Io::Output("Enter y coordinate:\n".to_string()))
        .expect(Io::Input("1\n".to_string()))
        .expect(Io::Input("3\n".to_string()));

    menu(&mut ui as &mut Ui);
    assert!(true)
}
