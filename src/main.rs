use std::{
    borrow::Borrow,
    error::Error,
    fs::{self, File},
    io::{BufReader, Read, Seek, SeekFrom},
};

fn read_previous_line<R>(reader: &mut BufReader<R>) -> Result<String, std::io::Error>
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

fn check_eof<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<(), Box<dyn Error>> {
    loop {
        let line = match read_previous_line(reader) {
            Err(e) => return Err(e.into()),
            Ok(line) => line,
        };
        if line.starts_with("%%EOF") {
            break;
        };
    }

    Ok(())
}

fn read_pdf(name: &String) {
    let file = File::open(name).unwrap();

    let mut reader = BufReader::new(file);
    reader.seek(SeekFrom::End(-2)).unwrap();
    if let Err(_) = check_eof(&mut reader) {
        dbg!("failed to check eof skipped...");
        return;
    }
    dbg!(read_previous_line(&mut reader).unwrap());
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
