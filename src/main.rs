use std::{
    fs::{self, File},
    io::BufReader,
};

mod parser;
mod utils;

fn read_pdf(name: &String) {
    let file = File::open(name).unwrap();

    let mut reader = BufReader::new(file);

    let table = match parser::read_xref_table(&mut reader) {
        Err(e) => {
            dbg!(e);
            return;
        }
        Ok(t) => t,
    };

    if table.len() == 0 {
        println!("No xref table was found");
    } else {
        println!("Found a xref table of {} records", table.len());
    }
}

fn main() {
    let path = "./resources";
    let _: Vec<_> = fs::read_dir(path)
        .unwrap()
        .into_iter()
        .map(|v| v.unwrap().file_name().to_str().unwrap().to_string())
        .map(|v| {
            println!("========================================");
            println!("{}", &v);
            v
        })
        .filter(|v| v.contains(".pdf"))
        .map(|v| read_pdf(&format!("{}/{}", path, v)))
        .map(|_| println!("========================================"))
        .collect();
}
