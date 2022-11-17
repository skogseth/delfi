#![allow(dead_code)]
use std::path::Path;
// use serde::Serialize;

#[derive(Debug)]
pub struct Dataset<Iter: Iterator<Item = Data>, Data> {
    iterator: Iter,
}

impl<IntoIt, It, Data> From<IntoIt> for Dataset<It, Data>
where
    IntoIt: IntoIterator<Item = Data, IntoIter = It>,
    It: Iterator<Item = Data>,
{
    fn from(into_it: IntoIt) -> Self {
        Dataset { iterator: into_it.into_iter() }
    }
}

/*
impl<Iter: Iterator<Item = Data>, Data: Serialize> Dataset<Iter, Data> {
    fn save(self, filename: &Path) {
        // save that data to csv using csv writer
    }
}
*/

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

