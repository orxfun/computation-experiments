use binrw::BinRead;
use bytes::Bytes;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Cursor};
use std::sync::atomic::AtomicU64;

pub struct LengthDelimitedCodec;

impl LengthDelimitedCodec {
    pub fn new() -> Self {
        LengthDelimitedCodec {}
    }

    pub fn decode(&self, mut reader: impl BufRead) -> io::Result<Bytes> {
        let mut length_buf = [0; 2];
        reader.read_exact(&mut length_buf)?;

        let length = u16::from_be_bytes(length_buf) as usize;
        let mut buf = vec![0; length];
        buf[..2].copy_from_slice(&length_buf);

        reader.read_exact(&mut buf[2..])?;

        Ok(Bytes::from(buf))
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let rt = match args.is_empty() {
        true => 0,
        false => args[0].parse().unwrap(),
    };
    dbg!(rt);

    let codec = LengthDelimitedCodec::new();
    let file = File::open("./resources/data/s1415.bin").unwrap();
    let mut reader = BufReader::with_capacity(327_560, file);
    let counter = AtomicU64::new(0);

    rayon::ThreadPoolBuilder::new()
        .num_threads(rt)
        .build_global()
        .unwrap();

    println!("Hello, world!");
}
