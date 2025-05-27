use std::io::{self, BufWriter, Write};

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let out = match args.is_empty() {
        true => "y".to_string(),
        false => args.join(" "),
    };
    let mut writer = BufWriter::new(io::stdout().lock());
    loop {
        writeln!(writer, "{}", out)?;
        writer.flush()?;
    }
}
