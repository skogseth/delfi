/*!
Delfi is a crate which seeks to minimize the code needed to save your data to a csv-file.
No matter which form your data takes it should be easy to save it to csv!
The crate is centralized around two core concept:
- The [Datapoint]: A set of data elements which can be recorded to csv-format
- The [Dataset]: A collection of datapoints, optionally with labels

A quick example using std-arrays:
```
use delfi::Dataset;

let t = [0.0, 0.5, 1.0];
let x = [1.0, 2.0, 4.0];

let ds = Dataset::from_columns([t, x]).with_labels(["time", "length"]);
ds.save("./resources/data/examples/basic.csv").unwrap();
```

Here is a slighlty more complex example using ndarrays:
```
use delfi::dataset;
use ndarray::Array;

const N: usize = 1000;
let x = Array::linspace(0., 10., N+1);
let y = Array::logspace(10., 0., 2., N+1);

let dataset = dataset!{
    "x" => x,
    "y" => y,
};

let directory = std::fs::canonicalize("./resources/data/examples/").unwrap();
let filepath = directory.join("ndarray.csv");
dataset.save(&filepath).unwrap();
```
*/

/// Module containing implementations on the Dataset struct
pub mod dataset;

/// Module containing code related to the Datapoint trait
pub mod datapoint;

/**
A dataset is a collection of datapoints.

They can be constructed in a multitude of ways. Two common ways are:

From a set of datapoints:
```
use delfi::Dataset;

let dp1 = [0, 1, 2];
let dp2 = [8, 3, 5];
let dp3 = [5, 4, 3];
let ds = Dataset::from_datapoints([dp1, dp2, dp3]);
# // ds.save("./resources/data/examples/rows.csv").unwrap();
```

From columns of data:
```
use delfi::Dataset;

let t = [0, 1, 2, 3, 4, 5];
let x = [2, 3, 5, 8, 12, 17];
let ds = Dataset::from_columns([t, x]);
# // ds.save("./resources/data/examples/columns.csv").unwrap();
```

One can also add labels in a multitude of ways, the simplest being whilst constructing the dataset:
```
use delfi::Dataset;

let t = [0, 1, 2, 3, 4, 5];
let x = [2, 3, 5, 8, 12, 17];
let ds = Dataset::from_columns([t, x]).with_labels(["time", "length"]);
# ds.save("./resources/data/examples/columns.csv").unwrap();
```

This is equivalent to using the dataset-macro:
```
use delfi::dataset;

let t = [0, 1, 2, 3, 4, 5];
let x = [2, 3, 5, 8, 12, 17];
let ds = dataset!{
    "time" => t,
    "length" => x,
};
# ds.save("./resources/data/examples/macro.csv").unwrap();
```
*/
#[derive(Debug, Clone)]
pub struct Dataset<const COLS: usize, Data: Datapoint<COLS>> {
    labels: Option<[String; COLS]>,
    data: Vec<Data>,
}

/**
A datapoint is a collection of dataelements which can be recorded to the csv-format. 
*/
pub trait Datapoint<const N: usize> {
    fn record(&self) -> [String; N];
}

/**
Derive Datapoint trait for a given struct (named or unnamed).
Unit struct and enums are not supported.

```
use delfi::Datapoint;

#[derive(Datapoint)]
struct Count {
    ch: char,
    i: usize,
}
```
*/
pub use delfi_macros::Datapoint;

/**
Macro for creating a dataset from a set of labelled columns 
*/
#[macro_export]
macro_rules! dataset {
    ($($name:expr => $values:expr), + $(,)?) => {{
        delfi::Dataset::from_columns([$($values),+]).with_labels([$($name),+])
    }};
}
