# Markov Chains

> An implementation of text based [markov chains](https://en.wikipedia.org/wiki/Markov_chain) in Rust. 

The main use of this is for the generation of text, and thus is not expandable for other uses. If you do need a more geeric implementation / library, please see the [markov](https://crates.io/crates/markov) or [markov-chain](https://crates.io/crates/markov-chain) crates.

## Example

```rust
fn main() {
    let tokens = include_str!("../input.txt")
        .split_whitespace()
        .map(|x| String::from(x))
        .collect();

    let cache = create_cache(tokens);
    let text = generate_text(cache, 500);
    println!("{}", text);
}
```

Please note that this library is not available as a crate as of yet, due to the non generic nature of the library.

This may or may not be changed in the near future.

## License

MIT.