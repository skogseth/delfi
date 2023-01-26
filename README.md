# Delfi
Rust crate for conveniently writing data to csv-files. The crate aims to be a minimal interface for saving data to csv such that this is not a considerable part of your program, preferably only taking up a few lines of code.

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

## Feature: macros
Work is currently being done to allow custom datastructures which combine multiple types. The current state allows patterns such as this (named structs are also supported):

```rust
use delfi::{Datapoint, Dataset};

#[derive(Datapoint)]
struct MyDatapoint(String, usize, f64);

let dp1 = MyDatapoint("Hello".to_owned(), 4, 10.2);
let dp2 = MyDatapoint("World".to_owned(), 5, 3.14);
let dataset = Dataset::from_datapoints([dp1, dp2]);

dataset.save("./path/to/file.csv").unwrap();
```

Hopefully this will be expanded upon in the future to allow for more ergonomic constructors.
