/*!
This crate is very much a work in progress!

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
pub struct Dataset<Iter: Iterator<Item = [Data; COLS]>, const COLS: usize, Data> {
    labels: Option<[String; COLS]>,
    data: Iter,
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
