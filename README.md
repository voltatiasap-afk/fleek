# fleek
sleek steganography tool written in rust

<p align="center">
  <img src="showcase.gif" alt="fleek demo" width="700">
</p>

## Usage
```text
Image steganography tool

Usage: fleek <COMMAND>

Commands:
  encode  encodes files or text to an image
  decode  decodes hidden data from images
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Encoding a file (of any format)
```bash
fleek encode -c carrier.png -f file.zip
```

## Decoding
```bash
fleek decode -i image.png
```
