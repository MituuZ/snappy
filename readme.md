# Snappy
Training tool for string and file manipulation used for picoCTF. Written in Rust by someone who doesn't yet know how to write Rust.

Contains full answers to some challenges, so be warned. No extensive comments or documentation.

## Creating executable
To make calling the program easy, you should build the executable and have it in your path.
This way you can call it from anywhere.

Here's an example of building and creating a link to freely call the program.

1. Clone the repository
```bash
git clone https://github.com/MituuZ/snappy.git
```

2. Build the executable
```bash
cd snappy
cargo build --release
```

3. Create a link to bin (or any directory that is included in the path)
```bash
cd target/release
ls -s $(pwd)/snappy /usr/local/bin/snappy # Or any other path that is in the path
```

## Calling the program
### From path
```bash
# String
snappy -r 'I say jump, you say how high'

# File
snappy -r -f text.txt

# stdin
echo 'Hello World' | snappy -r
```

### From the repository root
```bash
# String
cargo run -- -r 'I say jump, you say how high'

# File
cargo run -- -r -f text.txt

# stdin
echo 'Hello World' | cargo run -- -r
```

## Rot -r/--rot
Run all 26 rotation permutations for a file or a string.

```bash
cargo run -- -r 'I say jump, you say how high'
```

## Base64
### Encoding -be/--base-encode
Encode string in base64 encoding.

```bash
cargo run -- -be 'Pancake'
```

### Decoding -bd/--base-decode
Decode a base64 encoded string.
```bash
cargo run -- -bd 'UGFuY2FrZQ=='
```

