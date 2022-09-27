# Base64
This repository contains a base64 implementation written in rust.
## Usage
```bash
base64 0.1.0
Encodes supplied [text] as a String into base64 encoded text.

USAGE:
    base64 [OPTIONS] [text]

ARGS:
    <text>    

OPTIONS:
    -d, --decode <TEXT>    text to decode
    -w, --wrap <COLS>      wrap encoded lines after COLS value.
                           Use 0 to disable line wrapping. [default: 76]
    -h, --help             Print help information
    -V, --version          Print version information
```

## Todo: 
- [x] encoding
- [x] decoding
- [x] decoding flag (-d, --decode)
- [x] ignore garbage
- [ ] fail on garbage input
- [ ] ignore garbage flag (-i, --ignore-garbage)
- [x] version flag
- [x] default wrapping
- [x] custom column wrapping flag (-c, --cols=n)
- [ ] proper error handling
