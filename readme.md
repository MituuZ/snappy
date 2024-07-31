# Snappy
Training tool for string and file manipulation used for picoCTF. Written in Rust by someone who doesn't yet know how to write Rust.

## Calling the program
```bash
# String
cargo run -- -r 'I say jump, you say how high'

# File
cargo run -- -r -f text.txt

# stdin
echo 'Hello World' | cargo run -- -r
```

## Rot -r/--rot
Run rotation permutations for a file or a string.

```bash
cargo run -- -r 'I say jump, you say how high'
```

## Base64 Encoding
```bash
# String
cargo run -- -b 'I say jump, you say how high'
```
