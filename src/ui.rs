use std::io;

pub trait Ui {
    fn say(&mut self, buf: &String) -> ();
    fn ask(&mut self, buf: &mut String) -> ();
}

pub enum Io {
    Input(String),
    Output(String)
}

// struct RealUi;
//
// impl RealUi {
//     fn new() -> RealUi {
//         RealUi
//     }
// }
//
// impl Ui for RealUi {
//     fn ask(&mut self) -> String {
//
//     }
//
//     fn say(&mut self, buf: &mut String) -> () {
//         io::stdin().read_line(buf);
//     }
// }

pub struct MockUi {
    expectations: Vec<Io>,
    out: String
}

impl MockUi {
    pub fn new() -> MockUi {
        MockUi{
            expectations: Vec::new(),
            out: String::new()
        }
    }

    pub fn expect(&mut self, expecation: Io) -> &mut Self {
        self.expectations.insert(0, expecation);
        self
    }
}

impl Ui for MockUi {
    fn ask(&mut self, buf: &mut String) -> () {
        match self.expectations.pop().unwrap() {
            Io::Input(string) => buf.push_str(&string),
            Io::Output(_) => panic!("asked for output")
        }
    }

    fn say(&mut self, buf: &String) -> () {
        match self.expectations.pop().unwrap() {
            Io::Input(_) => panic!("found input"),
            Io::Output(string) => {
                assert_eq!(string, *buf);
                self.out.push_str(&string)
            }
        }
    }
}

#[test]
fn get_input_works() {
    let mut ui = MockUi::new();
    ui.expect(Io::Input("input string".to_string()));
    let mut string = String::new();
    ui.ask(&mut string);

    assert_eq!(string, "input string")
}

#[test]
fn input_and_output() {
    let mut ui = MockUi::new();
    ui
        .expect(Io::Output("hello".to_string()))
        .expect(Io::Input("hi".to_string()));


    ui.say(&"hello".to_string());
    let mut string = String::new();
    ui.ask(&mut string);
    assert_eq!(string, "hi")
}
