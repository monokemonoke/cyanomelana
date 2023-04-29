use std::{
    fs::{self, File},
    io::{BufReader, Seek, SeekFrom},
};

mod parser;
mod utils;

fn read_pdf(name: &String) {
    let file = File::open(name).unwrap();

    let mut reader = BufReader::new(file);

    parser::check_eof_with_limit(&mut reader, 16).unwrap();

    let xref_byte = parser::parse_xref_table_pos(&mut reader).unwrap();

    reader.seek(SeekFrom::Start(xref_byte)).unwrap();
    let table = match parser::parse_xref_table(&mut reader) {
        Err(e) => {
            eprintln!("{:?}", e);
            return;
        }
        Ok(t) => t,
    };
    dbg!(table);
}

fn main() {
    let path = "./resources";
    let _: Vec<_> = fs::read_dir(path)
        .unwrap()
        .into_iter()
        .map(|v| v.unwrap().file_name().to_str().unwrap().to_string())
        .map(|v| {
            println!("{}", &v);
            v
        })
        .filter(|v| v.contains(".pdf"))
        .map(|v| read_pdf(&format!("{}/{}", path, v)))
        .collect();
}
