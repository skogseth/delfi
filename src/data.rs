use std::path::Path;
use serde::Serialize;

use crate::{Datapoint, Dataset};

impl<S: Serialize> Datapoint for S {}

impl<IntoIter, Iter, const COLS: usize, Data> From<IntoIter> for Dataset<Iter, COLS, Data> 
where 
    IntoIter: IntoIterator<Item = [Data; COLS], IntoIter = Iter>,
    Iter: Iterator<Item = [Data; COLS]>,
{
    fn from(vals: IntoIter) -> Self {
        Dataset {
            labels: None,
            data: vals.into_iter(),
        }
    }
}

// Data implements Display
impl<Iter: Iterator<Item = [Data; COLS]>, const COLS: usize, Data: std::fmt::Display> Dataset<Iter, COLS, Data> {
    pub fn save(self, filepath: &Path) -> Result<(), std::io::Error> {
        let mut writer = csv::Writer::from_path(filepath)?;
        if let Some(labels) = self.labels {
            writer.write_record(&labels)?;
        }
        for datapoint in self.data {
            let record = datapoint.iter().map(|x| format!("{}", x));
            writer.write_record(record)?;
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

