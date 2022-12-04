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
use std::path::Path;
use serde::Serialize;

pub trait Datapoint {}

impl<S: Serialize> Datapoint for S {}

#[derive(Debug)]
pub struct Dataset<Iter: Iterator<Item = Data>, Data> {
    iterator: Iter,
}

// impl IntoIterator for Dataset
impl<IntoIter, Iter, Data> From<IntoIter> for Dataset<Iter, Data>
where
    IntoIter: IntoIterator<Item = Data, IntoIter = Iter>,
    Iter: Iterator<Item = Data>,
{
    fn from(into_iterator: IntoIter) -> Self {
        Dataset { iterator: into_iterator.into_iter() }
    }
}

// Serialize functions
impl<Iter: Iterator<Item = Data>, Data: Serialize> Dataset<Iter, Data> {
    pub fn save_series(self, filepath: &Path) -> Result<(), std::io::Error> {
        let mut writer = csv::Writer::from_path(filepath)?;
        for datapoint in self.iterator {
            writer.serialize(datapoint)?;
        }
        writer.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;    

    #[test]
    fn from_iterator() {
        let iterator = [1, 2, 3].into_iter();
        let dataset = Dataset::from(iterator);
        println!("{:?}", dataset);
    }
    
    #[test]
    fn from_vec() {
        let vector = vec![1, 2, 3];
        let dataset = Dataset::from(vector);
        println!("{:?}", dataset);
    }
}

