use std::{fs::File, io::{Read, BufReader, Write}};

use clap::{ArgGroup, Parser};
use sha3::{Digest, Sha3_256};
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm,
    Nonce
};

#[derive(Parser, Debug)]
#[command(author, version, about = "This is a utility to hide messages inside img files in an encrypted format.", long_about = None)]
#[command(group(
    ArgGroup::new("mode")
        .required(true)
        .args(["encode", "decode"])
))]
struct Args {
    #[arg(short, long, requires = "picture")]
    encode: bool,

    #[arg(short, long)]
    decode: bool,

    #[arg(short = 'c', long)]
    pass_phrase: String,

    #[arg(short, long)]
    picture: Option<String>,

    #[arg(short, long)]
    infile: String,

    #[arg(short, long)]
    outfile: String,
}

const MASKS: [u8; 8] = [1, 2, 4, 8, 16, 32, 64, 128];

fn u8_to_bits(inp: u8) -> [u8; 8] {
    let mut out = [0u8; 8];
    for i in 0..8 {
        out[7-i] = (inp & MASKS[i]) >> i
    }

    out
}

fn bits_to_u8(bits: &[u8]) -> u8 {
    let mut out = 0;
    out |= bits[0];
    for bit in &bits[1..8] {
        out <<= 1;
        out |= bit;
    }

    out
}

fn set_to_lsb(src: Vec<u8>, dst: Vec<u8>) -> Vec<u8> {
    let mut out = Vec::new();
    let mut bits = Vec::new();

    for byt in src {
        bits.extend_from_slice(&u8_to_bits(byt));
    };

    let n = bits.len();
    for i in 0..dst.len() {
        if i < n {
            let out_bit = ((dst[i] >> 1) << 1) | bits[i];
            out.push(out_bit);
        } else {
            out.push(dst[i]);
        }
    }

    out
}

fn get_from_lsb(bytes: Vec<u8>) -> Vec<u8> {
    let mut n_bytes = Vec::new();
    let mut bytes = bytes.iter();

    for _ in 0..16 {
        let i = *bytes.next().unwrap();
        n_bytes.push(i & 1);
    }

    let n = u16::from_be_bytes([bits_to_u8(&n_bytes[0..8]), bits_to_u8(&n_bytes[8..16])]);
    let mut bits = Vec::new();

    for _ in 0..(n * 8) {
        bits.push(bytes.next().unwrap() & 1)
    }

    let out: Vec<_> = bits.chunks_exact(8).map(|chunk| bits_to_u8(chunk)).collect();
    out
}

fn main() {    
    let args = Args::parse();
    let mut hasher = Sha3_256::new();

    let mut infile = File::open(args.infile.clone()).unwrap();

    hasher.update(args.pass_phrase);
    let key = hasher.finalize();

    let cipher = Aes256Gcm::new(&key);
    let nonce = Nonce::from_slice(b"unique nonce");

    if args.encode {
        let mut indata = Vec::new();
        infile.read_to_end(&mut indata).unwrap();

        let picture = image::open(args.picture.unwrap()).unwrap();

        let mut ciphertext = cipher.encrypt(nonce, indata.as_ref()).unwrap();
        let len = (ciphertext.len() as u16).to_be_bytes();

        ciphertext.insert(0, len[0]);
        ciphertext.insert(1, len[1]);

        image::save_buffer(args.outfile, &set_to_lsb(ciphertext, picture.clone().into_bytes()), picture.width(), picture.height(), picture.color()).unwrap();
    } else if args.decode {
        let buf_reader = BufReader::new(infile);
        let img = image::load(BufReader::new(buf_reader), image::ImageFormat::from_path(args.infile).unwrap()).unwrap();

        let ciphertext = get_from_lsb(img.into_bytes());

        let plaintext = cipher.decrypt(nonce, ciphertext.as_slice()).unwrap();

        let mut file = std::fs::File::create(args.outfile).unwrap();
        file.write_all(&plaintext).unwrap();
    }
}