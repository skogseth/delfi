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

but for now the data must be of the same type (and implement Default)

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
use delfi::dataset;

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

They function with anything iterable, e.g. ndarrays:
```
use delfi::dataset;
use ndarray::Array;

fn main() -> Result<(), std::io::Error> {
    const N: usize = 1000;
    let x = Array::linspace(0., 10., N+1);
    let y = Array::logspace(10., 0., 2., N+1);
    
    let data = dataset!{
        "x" => x,
        "y" => y,
    };
    // let filepath = std::fs::canonicalize("./data/test.csv")?;
    // data.save(&filepath)?;

    Ok(())
}
```

*/

pub mod dataset;
pub mod datapoint;

#[derive(Debug, Clone)]
pub struct Dataset<const COLS: usize, Data: Datapoint<COLS>> {
    labels: Option<[String; COLS]>,
    data: Vec<Data>,
}

pub trait Datapoint<const N: usize> {
    fn record(&self) -> [String; N];
}

#[macro_export]
macro_rules! dataset {
    ($($name:expr => $values:expr), + $(,)?) => {{
        delfi::Dataset::columns([$($values),+], [$($name),+])
    }};
}
