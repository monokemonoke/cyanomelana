use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Seek, SeekFrom},
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
    let mut buf = String::new();

    reader.read_line(&mut buf).unwrap();
    if !buf.starts_with("xref") {
        return;
    }
    buf.clear();
    reader.read_line(&mut buf).unwrap();
    let num_of_objects_str = buf.split(' ').last().unwrap().trim_end();
    let num_of_objects: u64 = num_of_objects_str.parse().unwrap();
    println!("{}", num_of_objects);
    let mut xref_table: Vec<parser::XrefRecord> = Vec::new();
    for _ in 0..num_of_objects {
        let mut buf = String::new();
        reader.read_line(&mut buf).unwrap();
        let parts: Vec<&str> = buf.split_whitespace().collect();
        assert_eq!(parts.len(), 3);

        xref_table.push(parser::XrefRecord::new(
            parts[0].parse().unwrap(),
            parts[1].parse().unwrap(),
            parser::ObjType::new(parts[2]).unwrap(),
        ))
    }
    dbg!(xref_table);
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
