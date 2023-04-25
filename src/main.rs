use std::{
    fs::{self, File},
    io::{BufReader, Read, Seek, SeekFrom},
};

fn read_pdf(name: &String) {
    const BUF_SIZE: usize = 1;
    let file = File::open(name).unwrap();

    let mut reader = BufReader::new(file);
    reader.seek(SeekFrom::End(-1)).unwrap();

    let mut buf = [0; BUF_SIZE];
    for _ in 0..2 {
        let mut line = String::new();
        loop {
            reader.read(&mut buf).unwrap();
            if let Err(_) = String::from_utf8(buf.to_vec()) {
                break;
            }
            if let Ok(c) = String::from_utf8(buf.to_vec()) {
                if c == "\n" {
                    break;
                }
                line = format!("{}{}", c, line);
            }
            reader.seek(SeekFrom::Current(-2)).unwrap();
        }
        dbg!(name, line);
        reader.seek(SeekFrom::Current(-2)).unwrap();
    }
}

fn main() {
    let _: Vec<_> = fs::read_dir(".")
        .unwrap()
        .into_iter()
        .map(|v| v.unwrap().file_name().to_str().unwrap().to_string())
        .filter(|v| v.contains(".pdf"))
        .map(|v| read_pdf(&v))
        .collect();
}
