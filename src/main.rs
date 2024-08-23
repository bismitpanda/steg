mod bits;
mod cmd;

use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    AeadCore, Aes256Gcm, Nonce,
};
use clap::Parser;
use image::ImageFormat;
use sha2::{Digest, Sha256};

use bits::{convert_from_bits, convert_to_bits};

fn extract_lsb(bytes: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();

    for &byte in bytes {
        out.push(byte & 1);
    }

    out
}

fn modify_lsb(bytes: &mut [u8], bits: &[u8]) {
    for (i, bit) in bits.iter().enumerate() {
        if *bit == 1 {
            bytes[i] |= 1;
        } else {
            bytes[i] &= !1;
        }
    }
}

fn main() {
    let args = cmd::Command::parse();

    match args.action {
        cmd::Actions::Encode(args) => {
            let img_format =
                ImageFormat::from_path(&args.image_path).expect("Could not find extension.");
            match img_format {
                ImageFormat::Bmp | ImageFormat::Png | ImageFormat::Tiff | ImageFormat::Ico => {
                    let img =
                        image::open(&args.image_path).expect("Invalid/Not supported image file");
                    let mut image_bytes = img.clone().into_bytes();

                    let input =
                        std::fs::read(args.infile).expect("Cannot read provided input file");

                    if image_bytes.len() < (input.len() + 6) * 8 {
                        println!("Provided input is too large to be stored.");
                    }

                    let input = args.key.map_or_else(
                        || [&[0], input.as_slice()].concat(),
                        |key| {
                            let key = Sha256::digest(key);
                            let cipher = Aes256Gcm::new(&key);
                            let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

                            let encrypted_input = cipher
                                .encrypt(&nonce, input.as_slice())
                                .expect("Could not encrypt input");

                            [&[1], nonce.as_slice(), encrypted_input.as_slice()].concat()
                        },
                    );

                    let input_bits = convert_to_bits(&input);
                    modify_lsb(&mut image_bytes, &input_bits);

                    image::save_buffer(
                        &args.outfile,
                        &image_bytes,
                        img.width(),
                        img.height(),
                        img.color(),
                    )
                    .expect("Could not save modified image buffer");
                }

                ImageFormat::Gif => todo!("GIF files are currently not supported."),
                ImageFormat::Jpeg => todo!("JPEG files are currently not supported."),

                _ => {
                    panic!("Image format not supported.")
                }
            }
        }

        cmd::Actions::Decode(args) => {
            let img_format =
                ImageFormat::from_path(&args.infile).expect("Could not find extension.");
            match img_format {
                ImageFormat::Bmp | ImageFormat::Png | ImageFormat::Tiff | ImageFormat::Ico => {
                    let img = image::open(&args.infile).expect("Could not open input file");
                    let image_bytes = img.as_bytes();

                    let lsb_bits = extract_lsb(image_bytes);

                    let input_data = convert_from_bits(&lsb_bits);
                    let (encrypted, rest) = input_data.split_first().expect("Found no data");

                    let data = match (*encrypted == 1, args.key) {
                        (true, Some(key)) => {
                            let (nonce_slice, ciphertext) = rest.split_at(12);
                            let key = Sha256::digest(key);
                            let cipher = Aes256Gcm::new(&key);
                            let nonce = Nonce::from_slice(nonce_slice);

                            cipher
                                .decrypt(nonce, ciphertext)
                                .expect("Could not decrypt stored data")
                        }

                        (false, _) => rest.to_vec(),
                        (true, None) => panic!("The data is encrypted, requires a key to decrypt."),
                    };

                    std::fs::write(&args.outfile, data).expect("Could not write to output file");
                }

                ImageFormat::Gif => todo!("GIF files are currently not supported."),
                ImageFormat::Jpeg => todo!("JPEG files are currently not supported."),

                _ => {
                    panic!("Image format not supported.")
                }
            }
        }

        cmd::Actions::Calculate(args) => {
            let img_format =
                ImageFormat::from_path(&args.image_path).expect("Could not find extension.");
            match img_format {
                ImageFormat::Bmp | ImageFormat::Png | ImageFormat::Tiff | ImageFormat::Ico => {
                    let img =
                        image::open(args.image_path).expect("Invalid/Not supported image file");

                    let image_bytes = img.as_bytes();
                    let storage_capacity = image_bytes.len() / 8 - 6;

                    println!(
                        "{storage_capacity} bytes of data can be stored in the image provided."
                    );
                }

                ImageFormat::Gif => todo!("GIF files are currently not supported."),
                ImageFormat::Jpeg => todo!("JPEG files are currently not supported."),

                _ => {
                    panic!("Image format not supported.")
                }
            }
        }
    }
}
