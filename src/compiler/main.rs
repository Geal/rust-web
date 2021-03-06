use std::io;
use std::io::File;

mod lexer;
mod parser;
mod token;
mod view_writer;

fn get_file_contents() -> Result<~[u8], io::IoError> {
    let path = from_str::<Path>("src/compiler/index.rs.html").unwrap();

    File::open(&path).and_then(|file| {
        let mut file = file;
        file.read_to_end()
    })
}

fn main() {
    let contents = match get_file_contents() {
        Ok(contents) => std::str::from_utf8_owned(contents).expect("Non-utf8 source file"),
        Err(e) => fail!(e)
    };

    let mut lexer = lexer::Lexer::new(contents);
    let mut parser = parser::Parser::new(&mut lexer);
    parser.parse();

    println!("Lines: {}\nLast column: {}", parser.lexer.line, parser.lexer.column);

    //debug!("{}", parser.sections);

    view_writer::write_view(&Path::new("src/compiler/index.rs"), parser.sections.as_slice());
}
