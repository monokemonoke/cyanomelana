use std::io::{BufReader, Error, ErrorKind, Read, Seek, SeekFrom};

pub fn read_previous_line<R>(reader: &mut BufReader<R>) -> Result<String, Error>
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

pub fn check_eof_with_limit<R: Read + Seek>(
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

#[derive(Debug)]
pub enum ObjType {
    F,
    N,
}

impl ObjType {
    pub fn new(str: &str) -> Result<Self, ()> {
        match str {
            "f" => Ok(Self::F),
            "n" => Ok(Self::N),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct XrefRecord {
    _byte: u64,
    _generation: u64,
    _obj_type: ObjType,
}

impl XrefRecord {
    pub fn new(byte: u64, generation: u64, obj_type: ObjType) -> Self {
        XrefRecord {
            _byte: byte,
            _generation: generation,
            _obj_type: obj_type,
        }
    }
}
