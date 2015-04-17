use std::io::{BufStream, Cursor, Write, Read};
use std::ops::Deref;

pub struct BoardWriter<'a> {
    writer: &'a mut Write
}

impl<'a> BoardWriter<'a> {
    pub fn new(writer: &'a mut Write) -> BoardWriter {
        BoardWriter{writer: writer}
    }

    pub fn write(&mut self, string: String) {
        self.writer.write(&string.into_bytes()).unwrap();
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
    let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());

    {
        let mut writer = BoardWriter::new(&mut cursor as &mut Write);
        writer.write("yo yo".to_string());
    }

    assert_eq!(cursor.into_string(), "yo yo");
}
