# Snappy
Training tool for string and file manipulation used for picoCTF. Written in Rust by someone who doesn't yet know how to write Rust.

## Rot
Run rotation permutations for a file or a string.

```
# String
cargo run -- -r 'I say jump, you say how high'

# File
cargo run -- -r -f text.txt

# stdin
echo 'Hello World' | cargo run -- -r
```

