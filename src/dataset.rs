use std::path::Path;

use crate::Dataset;
use crate::Datapoint;

/**
use delfi::Dataset;

let dp1 = [0.5, 1.0, 2.4];
let dp2 = [2.1, 3.6, 5.3];
let dp3 = [4.1, 3.2, 2.2];

let _ = Dataset::from([dp1, dp2, dp3]); 
*/
impl<const COLS: usize, IntoIter, Iter, Data> From<IntoIter> for Dataset<COLS, Data> 
where 
    IntoIter: IntoIterator<Item = Data, IntoIter = Iter>,
    Iter: Iterator<Item = Data>,
    Data: Datapoint<COLS>,
{
    fn from(vals: IntoIter) -> Self {
        Dataset {
            labels: None,
            data: vals.into_iter().collect(),
        }
    }
}

/** 
Takes in a set of columns and creates a dataset from these. Labels can optionally be passed as well.

# Examples
```
use delfi::Dataset;

let t = [0, 1, 2, 3, 4, 5];
let x = [2, 3, 5, 8, 12, 17];
let _ = Dataset::columns([t, x], ["time", "length"]);
```

Labels can also be turned off

```
use delfi::Dataset;

let t = [0, 1, 2, 3, 4, 5];
let x = [2, 3, 5, 8, 12, 17];
let _ = Dataset::columns([t, x], None);
```

and technically they also accept to be passed via `Some(_)`, but why would you?

```
use delfi::Dataset;

let t = [0, 1, 2, 3, 4, 5];
let x = [2, 3, 5, 8, 12, 17];
let _ = Dataset::columns([&t, &x], Some(["time", "length"]));
```
*/
impl<const COLS: usize, DataElement: ToString> Dataset<COLS, [DataElement; COLS]> {
    pub fn columns<'a, IntoIter, Iter, Labels>(cols: [IntoIter; COLS], labels: Labels) -> Self 
    where
        IntoIter: IntoIterator<Item = DataElement, IntoIter = Iter>,
        Iter: Iterator<Item = DataElement>,
        Labels: Into<Option<[&'a str; COLS]>>,
    {
        let mut cols: [Iter; COLS] = match cols.into_iter().map(|x| x.into_iter()).collect::<Vec<Iter>>().try_into() {
            // This segment is matched because unwrap requires Debug to be implemented for IntoIterator
            Ok(val) => val,
            Err(_) => panic!("This should never be reached"),
        };
        let mut data = Vec::new();
        'outer: loop {
            let mut temp = Vec::with_capacity(COLS);
            for col in cols.iter_mut() {
                if let Some(data) = col.next() {
                    temp.push(data);
                } else {
                    break 'outer;
                }
            }
            let row: [DataElement; COLS] = match temp.try_into() {
                // This segment is matched because unwrap requires Debug to be implemented for DataElement
                Ok(val) => val,
                Err(_) => panic!("This should never be reached"),
            };
            data.push(row);
        }
        let data = data;

        let labels: Option<[String; COLS]> = labels.into().map(|labels| {
            labels.into_iter().map(|x| x.to_owned()).collect::<Vec<String>>().try_into().unwrap()
        }); 
        
        Dataset { labels, data }
    }
}

impl<const COLS: usize, Data: Datapoint<COLS>> Dataset<COLS, Data> {
    pub fn save<P: AsRef<Path>>(self, filepath: P) -> Result<(), std::io::Error> {
        let mut writer = csv::Writer::from_path(filepath)?;
        if let Some(labels) = self.labels {
            writer.write_record(&labels)?;
        }
        for datapoint in self.data {
            writer.write_record(datapoint.record())?;
        }
        writer.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;    

    #[test]
    fn from_array() {
        let iterator = [[1, 2], [3, 4], [5, 6]];
        let dataset = Dataset::from(iterator);
        println!("{:?}", dataset);
    }

    #[test]
    fn from_iterator() {
        let iterator = [[1, 2], [3, 4], [5, 6]].into_iter();
        let dataset = Dataset::from(iterator);
        println!("{:?}", dataset);
    }
    
    #[test]
    fn from_vec() {
        let vector = vec![[1, 2], [3, 4], [5, 6]];
        let dataset = Dataset::from(vector);
        println!("{:?}", dataset);
    }
}

