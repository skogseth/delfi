use std::path::Path;
use serde::Serialize;

use crate::{Datapoint, Dataset};

impl<S: Serialize> Datapoint for S {}

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

