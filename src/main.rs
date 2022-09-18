use std::io;
use base64::{Config, CharacterSet, read::DecoderReader};
use clap::Parser;
use libflate::gzip::Decoder;

#[derive(Debug, Parser)]
#[clap(
    name = "psudad",
    author,
    version,
    about = "PS Utility Decode and Deflate\n\nSmall utility to decode and deflate base64 gzip input.",
    long_about = None,
)]
struct Args {
    #[clap(value_name = "VALUE", help = "RFC3548 base64 encoded string representing gzipped data.")]
    input: String,
}

fn main() {
    if let Err(err) = process() {
        eprintln!("ERROR: Decode or deflate failed. Message: \"{:#}\"", err);
        std::process::exit(1);
    }
}

fn process() -> io::Result<()> {
    let args: Args = Args::parse();

    let mut source: Box<dyn io::Read> = match args.input.as_str() {
        "-" => Box::new(io::stdin()),
        _ => Box::new(args.input.as_bytes()),
    };

    return decode_and_write(&mut source);
}

fn get_deflate<'a, R: 'a + io::Read>(value: &'a mut R) -> io::Result<Decoder<DecoderReader<'a, R>>> {
    let inner: DecoderReader<R> = DecoderReader::new(
        value,
        Config::new(CharacterSet::Standard, true));

    return Decoder::new(inner);
}

fn decode_and_write<'a, R: 'a + io::Read>(value: &'a mut R) -> io::Result<()> {
    let mut reader: Decoder<DecoderReader<R>> = get_deflate(value)?;
    io::copy(&mut reader, &mut io::stdout())?;
    return Ok(());
}
