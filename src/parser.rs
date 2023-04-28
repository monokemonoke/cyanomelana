use std::io::{BufRead, BufReader, Error, ErrorKind, Read, Seek, SeekFrom};

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
#[allow(dead_code)]
pub struct XrefRecord {
    byte: u64,
    generation: u64,
    obj_type: ObjType,
}

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

pub fn parse_xref_table(reader: &mut BufReader<std::fs::File>) -> Result<Vec<XrefRecord>, ()> {
    let mut buf = String::new();
    if let Err(_) = reader.read_line(&mut buf) {
        return Err(());
    }
    if !buf.trim().starts_with("xref") {
        return Err(());
    }

    buf.clear();
    if let Err(_) = reader.read_line(&mut buf) {
        return Err(());
    }
    let len_objects: u64 = match buf.trim_end().split(' ').last() {
        None => return Err(()),
        Some(n) => match n.parse() {
            Err(_) => return Err(()),
            Ok(n) => n,
        },
    };

    let mut xref_table: Vec<XrefRecord> = Vec::new();
    for _ in 0..len_objects {
        let mut buf = String::new();
        if let Err(_) = reader.read_line(&mut buf) {
            return Err(());
        }

        let parts: Vec<&str> = buf.split_whitespace().collect();
        if parts.len() != 3 {
            return Err(());
        }

        let byte: u64 = match parts[0].parse() {
            Err(_) => return Err(()),
            Ok(n) => n,
        };
        let gen: u64 = match parts[1].parse() {
            Err(_) => return Err(()),
            Ok(n) => n,
        };
        let obj_type = match ObjType::new(parts[2]) {
            Err(_) => return Err(()),
            Ok(t) => t,
        };
        xref_table.push(XrefRecord {
            byte: byte,
            generation: gen,
            obj_type: obj_type,
        })
    }

    Ok(xref_table)
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_read_previous_line() {
        struct TestCase<'a> {
            src: &'a [u8],
            want: &'a str,
            pos: i64,
        }
        let tests = [
            // TestCase {
            //     src: b"hoge",
            //     want: "hoge",
            //     pos: -1,
            // },
            TestCase {
                src: b"hoge\n",
                want: "",
                pos: -1,
            },
        ];

        for t in tests {
            let cursor = Cursor::new(t.src);
            let mut reader = BufReader::new(cursor);
            let result = reader.seek(SeekFrom::End(t.pos));
            assert!(result.is_ok(), "Error {:?}", result);

            let got = read_previous_line(&mut reader);
            assert!(got.is_ok(), "Error {:?}", got);
            assert_eq!(t.want, got.unwrap());
        }
    }
}
