/*!
We want something along the lines of this:

```ignore
#[derive(Datapoint)]
struct MyData {
    tag: String,
    i: i32,
    x: f64,
}

fn main() {
    let tags = { ... };
    let ints = { ... };
    let xs = { ... };
    let ds = dataset!(MyData; tags, ints, xs);
    ds.save("./path/to/file.csv")
}
```

// Alternatively, inspired by polars

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

```C

```python
xs = [1, 2, 3]
for x in xs:
    print(x)

xs[0] == 1
```

*/
#![allow(dead_code)]
use std::path::Path;
use serde::Serialize;

pub trait Datapoint: Serialize {}

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
    fn save(self, filepath: &Path) -> Result<(), std::io::Error> {
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
    
    #[derive(Serialize)]
    #[derive(Debug)]
    struct Count {
        character: char,
        count: u32,
    }
    
    #[test]
    fn save_to_file() {
        let chars = vec!['h', 'l', 'j'];
        let numbers = vec![115, 83, 24];
        let zipped = std::iter::zip(chars, numbers);
        let data = zipped.map(|x| Count { character: x.0, count: x.1 } );
        let dataset = Dataset::from(data);
        let path = std::path::PathBuf::from("./data/test.csv");
        let result = dataset.save(&path);
        result.unwrap();
    }
}

