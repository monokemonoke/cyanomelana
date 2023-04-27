use std::{
    fs::{self, File},
    io::{BufReader, Error, ErrorKind, Read, Seek, SeekFrom},
};

fn read_previous_line<R>(reader: &mut BufReader<R>) -> Result<String, Error>
where
    R: Read + Seek,
{
    let mut line = String::new();
    loop {
        let mut buf = [0; 1];
        reader.read(&mut buf)?;
        let c = &String::from_utf8_lossy(&buf);

        if c == "\n" {
            break;
        }
        line.insert_str(0, c);
        reader.seek(SeekFrom::Current(-2))?;
    }
    reader.seek(SeekFrom::Current(-2))?;
    Ok(line.trim_end().to_owned())
}

fn check_eof_with_limit<R: Read + Seek>(
    reader: &mut BufReader<R>,
    limit: usize,
) -> Result<(), Error> {
    for _ in 0..limit {
        let line = read_previous_line(reader)?;
        if line.starts_with("%%EOF") {
            return Ok(());
        }
    }
    return Err(Error::new(ErrorKind::NotFound, "hoge"));
}

fn read_pdf(name: &String) {
    let file = File::open(name).unwrap();

    let mut reader = BufReader::new(file);
    reader.seek(SeekFrom::End(-2)).unwrap();
    if let Err(_) = check_eof_with_limit(&mut reader, 16) {
        println!("failed to check eof skipped...");
        return;
    }
    println!("{}", read_previous_line(&mut reader).unwrap());
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
