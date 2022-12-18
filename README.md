# Delfi
Rust crate for conveniently writing data to csv-files. The crate aims to be a minimal interface for saving data to csv such that this is not a considerable part of your program, preferably only taking up one to three lines of code.

## Usage
A basic use of delfi is shown below:

```rust
use delfi::Dataset;

let t = vec![0.0, 1.0, 2.0];
let x = vec![0.0, 2.0, 6.0];
let dataset = Dataset::columns([t, x], ["time", "length"]);
dataset.save("./path/to/file.csv").unwrap();
```

Alternatively you can use the macro for slightly longer, but perhaps more readable, code:

```rust
use delfi::dataset;

let t = vec![0.0, 1.0, 2.0];
let x = vec![0.0, 2.0, 6.0];
let dataset = dataset!{
    "time" => t,
    "length" => x,
};
dataset.save("./path/to/file.csv").unwrap();
```

It works with anything iterable, so long as they are the same type. Here is an example using ndarray:

```rust
use delfi::dataset;
use ndarray::Array;

const N: usize = 1000;
let t = Array::linspace(0., 10., N+1);
let x = Array::logspace(10., 0., 2., N+1);

let dataset = dataset!{
    "time" => t,
    "length" => x,
};

dataset.save("./path/to/file.csv").unwrap();
```
