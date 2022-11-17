#![allow(dead_code)]
use std::path::Path;
use serde::Serialize;

#[derive(Debug)]
pub struct Dataset<Iter: Iterator<Item = Data>, Data> {
    iterator: Iter,
}

impl<IntoIter, Iter, Data> From<IntoIter> for Dataset<Iter, Data>
where
    IntoIter: IntoIterator<Item = Data, IntoIter = Iter>,
    Iter: Iterator<Item = Data>,
{
    fn from(into_iterator: IntoIter) -> Self {
        Dataset { iterator: into_iterator.into_iter() }
    }
}

// Setialize functions
impl<Iter: Iterator<Item = Data>, Data: Serialize + std::fmt::Debug> Dataset<Iter, Data> {
    fn save(self, filepath: &Path) -> Result<(), std::io::Error> {
        let mut writer = csv::Writer::from_path(filepath)?;
        log::debug!("Writing to file: {:?}", filepath);
        for datapoint in self.iterator {
            log::debug!("Writing datapoint: {:?}", datapoint);
            writer.serialize(datapoint)?;
            log::debug!("Succesfull!");
        }
        log::debug!("Finished writing data, flushing");
        writer.flush()?;
        log::debug!("Write succesfull!");
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
        let path = std::path::PathBuf::from("test.csv");
        let result = dataset.save(&path);
        result.unwrap();
    }
}

