/*!
This crate is very much a work in progress, but it can be used to some extent already.
An example of how to save data to a csv-file can be seen below:

# Example
```
use serde::Serialize;
use delfi::Dataset;

#[derive(Serialize, Debug)]
struct Count {
    character: char,
    count: u32,
}

fn main() {
    let chars = vec!['h', 'l', 'j'];
    let numbers = vec![115, 83, 24];
    let zipped = std::iter::zip(chars, numbers);
    let data = zipped.map(|x| Count { character: x.0, count: x.1 } );
    let dataset = Dataset::from(data);
    let path = std::path::PathBuf::from("./data/test.csv");
    dataset.save_series(&path).unwrap();
}
```

We want something along the lines of this:

```ignore
fn main() {
    let tags = { ... };
    let ints = { ... };
    let xs = { ... };
    let ds = Dataset::columns([tags, ints, xs], ["tag", "int", "x"]);
    ds.save("./path/to/file.csv")
}
```

and this

```ignore
fn main() {
    let tags = { ... };
    let ints = { ... };
    let xs = { ... };
    let ds = dataset!{
        "tag" => tags,
        "i" => ints,
        "x" => xs,
    };
    ds.save("./path/to/file.csv")
}
```
*/
mod data;

pub trait Datapoint {}

#[derive(Debug)]
pub struct Dataset<Iter: Iterator<Item = Data>, Data> {
    iterator: Iter,
}

#[macro_export]
macro_rules! dataset {
    ($($name:expr => $values:expr), + $(,)?) => {{
        let mut data = vec![vec![$(String::from($name)),+]];
        $(
            let strings = $values.iter().map(|x| format!("{}", x)).collect::<Vec<_>>();
            data.push(strings);
        )+
        Dataset::from(data)
    }};
}
