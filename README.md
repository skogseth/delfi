# Delfi
Rust crate for conveniently writing data to csv-files. The crate aims to be a minimal interface for saving data to csv such that this is not a considerable part of your program, preferably only taking up one to three lines of code.

#Usage
A basic use of delfi is shown below:

```rust
use delfi::Dataset;

fn main() {
    let t = [1.0, 2.0, 3.0];
    let x = [0.0, 2.0, 4.0];
    let dataset = Dataset::columns([t, x], ["time", "length"]);
    dataset.save("./path/to/file.csv").unwrap();
}
```
