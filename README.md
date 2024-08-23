# Steg

Steg is a rust based tool for steganography of image files.

Currently Supports:
- **PNG**
- **TIFF**
- **BMP**
- **ICO**

# Note
The JPEG steganography uses code from [`image`](https://crates.io/crates/image) crate and [`jpeg-decoder`](https://crates.io/crates/jpeg-decoder) crate.

- The `src/jpeg` folder of `steg` is a direct copy of `src/codecs/jpeg` of the `image` crate with some changes for steganography.
- The `src/jpeg_decoder` folder of steg is a copy of `src` folder of the `jpeg-decoder` crate with some changes for steganography.

## Help

```
Usage: steg.exe <COMMAND>

Commands:
  e, encode     Encode a message in to an image
  d, decode     Extract the message stored in a message
  c, calculate  Calculate the possible storage capacity of the image
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```