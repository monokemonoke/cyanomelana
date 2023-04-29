use crate::utils;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read, Seek};

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

pub fn check_eof_with_limit<R: Read + Seek>(
    reader: &mut BufReader<R>,
    limit: usize,
) -> Result<(), Error> {
    for _ in 0..limit {
        let line = utils::read_previous_line(reader)?;
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
