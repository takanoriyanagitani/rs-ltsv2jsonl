use std::io;
use std::io::Write;

use std::process::ExitCode;

use std::env;

use rs_ltsv2jsonl::LTSV_FIELD_SEPARATOR_DEFAULT;
use rs_ltsv2jsonl::LTSV_VALUE_SEPARATOR_DEFAULT;

use rs_ltsv2jsonl::JSONL_NEWLINE_BYTE_DEFAULT;

fn stdin2stdout(field_sep: char, value_sep: char, newline: u8) -> Result<(), io::Error> {
    let i = io::stdin();
    let il = i.lock();

    let o = io::stdout();
    let mut ol = o.lock();
    rs_ltsv2jsonl::ltsv2json::reader2writer(il, &mut ol, field_sep, value_sep, newline)?;
    ol.flush()?;
    Ok(())
}

fn sub() -> Result<(), io::Error> {
    let field_separator: char = env::var("ENV_LTSV_FIELD_SEPARATOR")
        .ok()
        .and_then(|s| s.chars().nth(0))
        .unwrap_or(LTSV_FIELD_SEPARATOR_DEFAULT);
    let value_separator: char = env::var("ENV_LTSV_VALUE_SEPARATOR")
        .ok()
        .and_then(|s| s.chars().nth(0))
        .unwrap_or(LTSV_VALUE_SEPARATOR_DEFAULT);

    let newline_char: u8 = env::var("ENV_JSONL_NEWLINE_CHAR_BYTE")
        .ok()
        .and_then(|s| str::parse(s.as_str()).ok())
        .unwrap_or(JSONL_NEWLINE_BYTE_DEFAULT);

    stdin2stdout(field_separator, value_separator, newline_char)
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
