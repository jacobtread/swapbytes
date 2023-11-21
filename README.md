## Swapbytes

![License](https://img.shields.io/github/license/jacobtread/swapbytes?style=for-the-badge)
![Cargo Version](https://img.shields.io/crates/v/swapbytes?style=for-the-badge)
![Cargo Downloads](https://img.shields.io/crates/d/swapbytes?style=for-the-badge)

Rust library for swapping the endianess of a structure using a derive macro

## Cargo

Using `swapbytes` with cargo

```toml
[dependencies]
swapbytes = "0.2"
```
or 

```shell
cargo add swapbytes
```


```rust
use swapbytes::SwapBytes;

#[derive(SwapBytes)]
pub struct Test {
    pub a: u32,
    pub b: u32,
    /// Skip this field
    #[sb(skip)]
    pub b: String,
}

let mut value: Test = Test { a: 1, b: 4 };
value.swap_bytes_mut();


/* Enum must implement Clone, Copy */
#[derive(SwapBytes, Clone, Copy)]
#[repr(u32)] /* Only number repr types are supported */
pub enum ReprEnum {
    A = 1,
    B = 2
}

```