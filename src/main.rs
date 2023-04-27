use std::{
    fs::{self, File},
    io::{BufReader, Seek, SeekFrom},
};

mod parser;

fn read_pdf(name: &String) {
    let file = File::open(name).unwrap();

    let mut reader = BufReader::new(file);
    reader.seek(SeekFrom::End(-2)).unwrap();
    if let Err(_) = parser::check_eof_with_limit(&mut reader, 16) {
        return;
    }

    let xref_byte = parser::read_previous_line(&mut reader).unwrap();
    let xref_byte: u64 = xref_byte.parse().unwrap();

    reader.seek(SeekFrom::Start(xref_byte)).unwrap();
    let table = match parser::parse_xref_table(&mut reader) {
        Err(()) => return,
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
