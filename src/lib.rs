#![allow(dead_code)]
use std::path::Path;
use serde::Serialize;

#[derive(Debug)]
pub struct Dataset<Iter: Iterator<Item = Data>, Data> {
    iterator: Iter,
}

impl<Iter: Iterator<Item = Data>, Data> From<Iter> for Dataset<Iter, Data> {
    fn from(iterator: Iter) -> Self {
        Dataset { iterator }
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
        let data_vec = vec![1, 2, 3];
        let dataset = Dataset::from(data_vec.iter());
        println!("{:?}", dataset);
    }
}

