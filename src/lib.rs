/*!
This crate is very much a work in progress!
We want something along the lines of this

```ignore
fn main() {
    let tags = { ... };
    let ints = { ... };
    let xs = { ... };
    let ds = Dataset::columns([tags, ints, xs], ["tag", "int", "x"]);
    ds.save("./path/to/file.csv").unwrap();
}
```

but for now the data must be of the same type (and implement both Default and Display)

```
use delfi::Dataset;

fn main() {
    let t = [0, 1, 2, 3, 4, 5];
    let x = [2, 3, 5, 8, 12, 17];
    let ds = Dataset::columns([t, x], ["time", "length"]);
    // ds.save("./data/examples/columns.csv").unwrap();
}
```

Alternatively, using the macro:

```
use delfi::{Dataset, dataset};

fn main() {
    let t = [0, 1, 2, 3, 4, 5];
    let x = [2, 3, 5, 8, 12, 17];
    let ds = dataset!{
        "time" => t,
        "length" => x,
    };
    // ds.save("./data/examples/macros.csv").unwrap();
}
```
*/
mod dataset;

pub trait Datapoint {}

#[derive(Debug)]
pub struct Dataset<Iter: Iterator<Item = [Data; COLS]>, const COLS: usize, Data> {
    labels: Option<[String; COLS]>,
    data: Iter,
}

#[macro_export]
macro_rules! dataset {
    ($($name:expr => $values:expr), + $(,)?) => {{
        delfi::Dataset::columns([$($values),+], [$($name),+])
    }};
}
