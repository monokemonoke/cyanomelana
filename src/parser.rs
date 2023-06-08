use crate::utils;
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

pub fn read_xref_table<R: Read + Seek>(
    reader: &mut BufReader<R>,
) -> Result<Vec<XrefRecord>, Error> {
    const LIMIT: usize = 16;

    check_eof_with_limit(reader, LIMIT)?;
    let xref_byte = parse_xref_table_pos(reader)?;

    reader.seek(SeekFrom::Start(xref_byte))?;
    parse_xref_table(reader)
}

/// check `%%EOF` comment in PDF file
fn check_eof_with_limit<R: Read + Seek>(
    reader: &mut BufReader<R>,
    limit: usize,
) -> Result<(), Error> {
    if reader.seek(SeekFrom::End(-1)).is_err() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "cannot seek to the last of the file",
        ));
    }
    for _ in 0..limit {
        let line = utils::read_previous_line(reader)?;
        if line.starts_with("%%EOF") {
            return Ok(());
        }
    }
    return Err(Error::new(
        ErrorKind::NotFound,
        "cannot find EOF comment in PDF",
    ));
}

/// parse xref table position after find `%%EOF` comment
fn parse_xref_table_pos<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<u64, Error> {
    let xref_byte = utils::read_previous_line(reader)?;
    match xref_byte.parse::<u64>() {
        Err(_) => Err(Error::new(
            ErrorKind::InvalidData,
            "cannot parse xref table position",
        )),
        Ok(n) => Ok(n),
    }
}

/// parse xref table content as a `Vec<XrefRecord>`
fn parse_xref_table<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Vec<XrefRecord>, Error> {
    let mut buf = [0; 4];

    // parse `xref` token
    reader.read(&mut buf)?;
    if !buf.starts_with(b"xref") {
        reader.seek(SeekFrom::Current(-4))?;
    }

    // skip CRLF tokens
    let mut buf = [0; 1];
    loop {
        reader.read(&mut buf)?;
        if &buf != b"\n" && &buf != b"\r" {
            reader.seek(SeekFrom::Current(-1))?;
            break;
        }
    }

    // parse number of objects
    let mut buf = String::new();
    reader.read_line(&mut buf)?;
    let num_of_objects: u64 = match buf.trim().split(' ').nth(1) {
        None => {
            return Err(Error::new(
                ErrorKind::NotFound,
                "cannot find the number of objects",
            ))
        }
        Some(str) => match str.parse() {
            Err(_) => {
                return Err(Error::new(
                    ErrorKind::NotFound,
                    "cannot find the number of objects",
                ))
            }
            Ok(n) => n,
        },
    };

    // parse table contents
    let mut table: Vec<XrefRecord> = Vec::new();
    for _ in 0..num_of_objects {
        let mut buf = String::new();
        reader.read_line(&mut buf)?;

        let err_failed_to_parse: Error =
            Error::new(ErrorKind::NotFound, "cannot find the number of objects");

        let byte: u64 = match buf.split(' ').nth(0) {
            None => return Err(err_failed_to_parse),
            Some(s) => match s.parse() {
                Err(_) => return Err(err_failed_to_parse),
                Ok(b) => b,
            },
        };
        let gen: u64 = match buf.split(' ').nth(1) {
            None => return Err(err_failed_to_parse),
            Some(s) => match s.parse() {
                Err(_) => return Err(err_failed_to_parse),
                Ok(b) => b,
            },
        };
        let obj_type = match buf.split(' ').nth(2) {
            None => return Err(err_failed_to_parse),
            Some(s) => match ObjType::new(s) {
                Err(_) => return Err(err_failed_to_parse),
                Ok(t) => t,
            },
        };

        table.push(XrefRecord {
            byte: byte,
            generation: gen,
            obj_type: obj_type,
        })
    }
    Ok(table)
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_check_eof_with_limit() {
        let cursor = Cursor::new(b"%%EOF");
        let mut reader = BufReader::new(cursor);
        let res = check_eof_with_limit(&mut reader, 1);
        assert!(res.is_ok(), "want ok but got Err({:?})", res.err());

        let cursor = Cursor::new(b"%%EOF\r\n");
        let mut reader = BufReader::new(cursor);
        let res = check_eof_with_limit(&mut reader, 4);
        assert!(res.is_ok(), "want ok but got Err({:?})", res.err());

        let cursor = Cursor::new(b"%%EOFother messy strings");
        let mut reader = BufReader::new(cursor);
        let res = check_eof_with_limit(&mut reader, 4);
        assert!(res.is_ok(), "want ok but got Err({:?})", res.err());
    }

    #[test]
    fn test_check_eof_with_limit_for_empty_file() {
        let cursor = Cursor::new(b"");
        let mut reader = BufReader::new(cursor);

        let res = check_eof_with_limit(&mut reader, 1);
        assert!(res.is_err(), "want Err but got {:?}", res);
    }

    #[test]
    fn test_parse_xref_table_pos() {
        let cursor = Cursor::new(b"1234");
        let mut reader = BufReader::new(cursor);
        reader.seek(SeekFrom::End(-1)).unwrap();

        let res = parse_xref_table_pos(&mut reader);
        assert!(res.is_ok(), "want Ok but got Err({:?})", res.err());
        let got = res.unwrap();
        assert_eq!(got, 1234, "want 1234 but got {}", got);
    }
}
