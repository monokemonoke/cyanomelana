use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Error, ErrorKind, Read, Seek, SeekFrom},
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
        return;
    }

    let xref_byte = read_previous_line(&mut reader).unwrap();
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
    for _ in 0..num_of_objects {
        let mut buf = String::new();
        reader.read_line(&mut buf).unwrap();
        let parts: Vec<&str> = buf.split_whitespace().collect();
        let object_id = parts[0];
        let generation = parts[1];
        let byte = parts[2];

        println!("{} {} {}", object_id, generation, byte);
    }
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
