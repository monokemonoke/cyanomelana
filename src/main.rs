use std::{
    error::Error,
    fs::{self, File},
    io::{BufReader, Read, Seek, SeekFrom},
};

fn read_previous_line<R>(reader: &mut BufReader<R>) -> Result<String, Box<dyn Error>>
where
    R: Read + Seek,
{
    let mut line = String::new();
    loop {
        let mut buf = [0; 1];
        reader.read(&mut buf).unwrap();
        let c = match String::from_utf8(buf.to_vec()) {
            Err(_) => {
                reader.seek(SeekFrom::Current(-2)).unwrap();
                return Ok(line);
            }
            Ok(c) => c,
        };

        match c.as_str() {
            "\n" => {
                reader.seek(SeekFrom::Current(-2)).unwrap();
                return Ok(line);
            }
            _ => (),
        }
        line = format!("{}{}", c, line);
        reader.seek(SeekFrom::Current(-2)).unwrap();
    }
}

fn read_pdf(name: &String) {
    let file = File::open(name).unwrap();

    let mut reader = BufReader::new(file);
    reader.seek(SeekFrom::End(-2)).unwrap();

    for _ in 0..2 {
        let line = read_previous_line(&mut reader).unwrap();
        dbg!(line);
    }
}

fn main() {
    let path = "./resources";
    let _: Vec<_> = fs::read_dir(path)
        .unwrap()
        .into_iter()
        .map(|v| v.unwrap().file_name().to_str().unwrap().to_string())
        .map(|v| {
            dbg!(&v);
            v
        })
        .filter(|v| v.contains(".pdf"))
        .map(|v| read_pdf(&format!("{}/{}", path, v)))
        .collect();
}
