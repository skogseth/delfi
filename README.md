# Delfi
Rust crate for conveniently writing data to csv-files. The crate aims to be a minimal interface for saving data to csv such that this is not a considerable part of your program, preferably only taking up one to three lines of code.

#Usage
A basic use of delfi is shown below:

```rust
use delfi::Dataset;

fn main() {
    let t = vec![0.0, 1.0, 2.0];
    let x = vec![0.0, 2.0, 6.0];
    let dataset = Dataset::columns([t, x], ["time", "length"]);
    dataset.save("./path/to/file.csv").unwrap();
}
```

Alternatively you can use the macro for slightly longer, but perhaps more readable, code:

```rust
use delfi::dataset;

fn main() {
    let t = vec![0.0, 1.0, 2.0];
    let x = vec![0.0, 2.0, 6.0];
    let dataset = dataset!{
        "time" => t,
        "length" => x,
    };
    dataset.save("./path/to/file.csv").unwrap();
}
```
