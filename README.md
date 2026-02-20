# fleek

sleek steganography tool for images written in rust

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
fleek encode -m carrier.png -f file.zip -c 5
```

### Decoding

```bash
fleek decode -i image.png
```

## Try it

```bash
fleek decode -i assets/source.png
```

This image contains the whole program's source code in a zip file
