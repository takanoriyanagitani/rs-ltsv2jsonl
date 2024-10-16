use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;

use serde_json::Map;
use serde_json::Value;

pub fn ltsv2map2writer<S, W>(
    ltsv: S,
    buf: &mut Map<String, Value>,
    wtr: &mut W,
    field_separator: char,
    value_separator: char,
    newline_byte: u8,
) -> Result<(), io::Error>
where
    S: AsRef<str>,
    W: Write,
{
    buf.clear();

    let line: &str = ltsv.as_ref();
    let pairs = line.split(field_separator);
    for pair in pairs {
        let mut splited = pair.splitn(2, value_separator);
        let olabel: Option<&str> = splited.next();
        match olabel {
            None => {}
            Some(label) => {
                let value: &str = splited.next().unwrap_or_default();
                buf.insert(label.into(), Value::String(value.into()));
            }
        }
    }

    serde_json::to_writer(wtr.by_ref(), &buf)?;

    wtr.write_all(&[newline_byte])?;

    Ok(())
}

pub fn ltsv2map2writer_default<S, W>(
    ltsv: S,
    buf: &mut Map<String, Value>,
    wtr: &mut W,
) -> Result<(), io::Error>
where
    S: AsRef<str>,
    W: Write,
{
    ltsv2map2writer(ltsv, buf, wtr, '\t', ':', b'\n')
}

pub fn lines2writer<I, W>(
    lines: I,
    wtr: &mut W,
    field_separator: char,
    value_separator: char,
    newline_byte: u8,
) -> Result<(), io::Error>
where
    I: Iterator<Item = String>,
    W: Write,
{
    let mut buf: Map<String, Value> = Map::new();
    for line in lines {
        ltsv2map2writer(
            line,
            &mut buf,
            wtr,
            field_separator,
            value_separator,
            newline_byte,
        )?;
    }
    wtr.flush()?;
    Ok(())
}

pub fn reader2writer<R, W>(
    rdr: R,
    wtr: &mut W,
    field_separator: char,
    value_separator: char,
    newline_byte: u8,
) -> Result<(), io::Error>
where
    R: Read,
    W: Write,
{
    let br = BufReader::new(rdr);
    let lines = br.lines();
    let noerr = lines.map_while(Result::ok);
    {
        let mut bw = BufWriter::new(wtr.by_ref());
        lines2writer(
            noerr,
            &mut bw,
            field_separator,
            value_separator,
            newline_byte,
        )?;
        bw.flush()?;
    }
    wtr.flush()?;
    Ok(())
}
