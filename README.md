# dtypei

Build interface description objects for your Rust exports.

This is a work-in-progress. The current version has no safety-guarantees.
Suggestions and contributions are welcome.

Short video presentation: https://youtu.be/tkbo-cnlCKk - using `dtypei` built interfaces to create programmatic UIs & enforce proper type checking for Wasm modules, in JavaScript.

## Usage

You need both https://crates.io/crates/dtypei and https://crates.io/crates/dtypei-derive.

```
#[macro_use]
extern crate dtypei_derive;

use dtypei;

#[dtypei_attr()]
pub fn sum(n1: i32, n2: i32) -> i32 {
    n1 + n2
}

#[dtypei_enum()]
pub enum Material {
    Wood,
    Metal,
    Plastic,
}

#[dtypei_struct()]
pub struct WoodFinish {
    pub sanded: bool,
    pub varnished: bool,
    pub polished: bool,
}

#[dtypei_struct()]
pub struct Table {
    pub id: u64,
    pub legs: u8,
    pub material: Material,
    pub finish: WoodFinish,
}

#[dtypei_attr()]
pub fn polish(mut table: Table) -> Table {
    table.finish.polished = true;
    table
}

#[dtypei_attr()]
pub fn typedinterface() -> Vec<dtypei::Typei> {

}
```

The `dtypei_attr` macro will populate the `typedinterface` function with the interface objects. E.g.:

```
[Typei { name: "sum", inputs: [SubTypes { name: "i32", label: "n1", dimensions: [] }, SubTypes { name: "i32", label: "n2", dimensions: [] }], outputs: [SubTypes { name: "sum", label: "i32", dimensions: [] }] }]
```

### Wasm Support

When using `#[wasm_bindgen]`, use this macro order:
```
#[wasm_bindgen]
#[dtypei_attr()]
pub fn sum(n1: i32, n2: i32) -> i32 {
    n1 + n2
}
```

### Examples

Check `./tests` examples.

```
cd tests/dtypeitest
cargo build
cargo run target/debug/dtypeitest
```

Wasm test:

```
cd tests/mathi
wasm-pack build

cd tests/www
npm install
npm run start
```

Check browser logs.

GPLv3 - see `./LICENSE`
