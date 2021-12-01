# build-features
A simple proc macro to create a vector that a list of enabled features in a root package

`Note: using this crate makes your build non-deterministic!`

## Usage

```rust
fn main() {
    build_features::get_enabled_features!();
    println!("{:?}", enabled_features); //enjoy!
}
```

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License
[MIT](https://choosealicense.com/licenses/mit/)
