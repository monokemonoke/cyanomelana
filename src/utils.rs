use std::io::{BufReader, Error, Read, Seek, SeekFrom};

pub fn read_previous_line<R>(reader: &mut BufReader<R>) -> Result<String, Error>
where
    R: Read + Seek,
{
    let mut bytes: Vec<u8> = Vec::new();
    loop {
        let mut buf = [0; 1];
        reader.read(&mut buf)?;

        match &buf {
            b"\n" | b"\r" => {
                if reader.stream_position()? >= 2 {
                    reader.seek(SeekFrom::Current(-2))?;
                }
                while reader.stream_position()? >= 2 {
                    reader.read(&mut buf)?;
                    if &buf != b"\r" && &buf != b"\n" {
                        reader.seek(SeekFrom::Current(-1))?;
                        break;
                    }
                    reader.seek(SeekFrom::Current(-2))?;
                }
                break;
            }
            _ => {
                println!("{}", String::from_utf8_lossy(&buf));
                bytes.push(buf[0]);
                if reader.stream_position()? < 2 {
                    break;
                }
                reader.seek(SeekFrom::Current(-2))?;
            }
        }
    }

    bytes.reverse();
    let str = String::from_utf8_lossy(&bytes)
        .to_string()
        .trim_end()
        .to_owned();

    Ok(str)
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_read_previous_line() {
        let cursor = Cursor::new(b"fuga\r\nfoo\nhoge");
        let mut reader = BufReader::new(cursor);
        reader.seek(SeekFrom::End(-1)).unwrap();

        let res = read_previous_line(&mut reader);
        assert!(res.is_ok(), "want Ok but got Err({:?})", res.err());
        assert_eq!(res.unwrap(), "hoge".to_string());

        let res = read_previous_line(&mut reader);
        assert!(res.is_ok(), "want Ok but got Err({:?})", res.err());
        assert_eq!(res.unwrap(), "foo".to_string());

        let res = read_previous_line(&mut reader);
        assert!(res.is_ok(), "want Ok but got Err({:?})", res.err());
        assert_eq!(res.unwrap(), "fuga".to_string());
    }
}
