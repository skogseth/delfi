/*!
Implementations on the [Dataset] struct
*/

use std::path::Path;

use crate::Datapoint;
use crate::Dataset;

impl<const COLS: usize, Data: Datapoint<COLS>> Dataset<COLS, Data> {
    /**
    Function for creating new (empty) dataset
    */
    #[must_use]
    pub fn new() -> Self {
        Self {
            labels: None,
            data: Vec::new(),
        }
    }

    /**
    Push a new row to the dataset
    */
    pub fn push(&mut self, datapoint: Data) {
        self.data.push(datapoint);
    }

    /**
    Get current number of rows in dataset, which is equal to the number of datapoints, plus 1 if there is a header row
    */
    #[must_use]
    pub fn n_rows(&self) -> usize {
        match self.labels {
            Some(_) => self.data.len() + 1,
            None => self.data.len(),
        }
    }

    /**
    Get current number of rows in dataset, which is equal to the number of datapoints, plus 1 if there is a header row
    */
    #[must_use]
    pub fn n_datapoints(&self) -> usize {
        self.data.len()
    }

    /**
    Get current number of rows in dataset
    */
    #[must_use]
    pub fn n_columns(&self) -> usize {
        COLS
    }

    /**
    Get current labels
    */
    #[must_use]
    pub fn get_labels(&self) -> Option<&[String; COLS]> {
        self.labels.as_ref()
    }

    /**
    Set labels for the given dataset.
    Constructors return dataset with labels set to None unless otherwise specified.

    ```
    use delfi::Dataset;

    let t = [0, 1, 2, 3, 4, 5];
    let x = [2, 3, 5, 8, 12, 17];
    let mut dataset = Dataset::from_columns([t, x]);
    dataset.set_labels(["time", "length"]);
    ```

    Labels can also be turned off

    ```
    # use delfi::Dataset;
    #
    # let t = [0, 1, 2, 3, 4, 5];
    # let x = [2, 3, 5, 8, 12, 17];
    # let mut dataset = Dataset::from_columns([t, x]);
    # dataset.set_labels(["time", "length"]);
    #
    dataset.set_labels(None);
    ```

    They also technically accept labels to be passed via `Some(_)` (but why would you?):

    ```
    # use delfi::Dataset;
    #
    # let t = [0, 1, 2, 3, 4, 5];
    # let x = [2, 3, 5, 8, 12, 17];
    # let mut dataset = Dataset::from_columns([t, x]);
    #
    dataset.set_labels(Some(["time", "length"]));
    ```
    */
    pub fn set_labels<'a, Labels>(&mut self, labels: Labels)
    where
        Labels: Into<Option<[&'a str; COLS]>>,
    {
        let labels: Option<[String; COLS]> = labels.into().map(|labels| {
            labels
                .into_iter()
                .map(ToOwned::to_owned)
                .collect::<Vec<String>>()
                .try_into()
                .expect("Failed to coerce vec into array")
        });
        self.labels = labels;
    }

    /**
    Take dataset, set labels, and return dataset. Useful when constructing datasets.

    ```
    use delfi::Dataset;

    let t = [0, 1, 2, 3, 4, 5];
    let x = [2, 3, 5, 8, 12, 17];
    let _ = Dataset::from_columns([&t, &x]).with_labels(["time", "length"]);
    ```

    See set_labels() for detail on possible parameters.
    */
    #[must_use]
    pub fn with_labels<'a, Labels>(mut self, labels: Labels) -> Self
    where
        Labels: Into<Option<[&'a str; COLS]>>,
    {
        self.set_labels(labels);
        self
    }

    /**
    Create a dataset from an iterator over datapoints
    */
    #[must_use]
    pub fn from_datapoints<IntoIter, Iter>(rows: IntoIter) -> Self
    where
        IntoIter: IntoIterator<Item = Data, IntoIter = Iter>,
        Iter: Iterator<Item = Data>,
        Data: Datapoint<COLS>,
    {
        Self {
            labels: None,
            data: rows.into_iter().collect(),
        }
    }
}

/**
Default is equivalent to new
*/
impl<const COLS: usize, Data: Datapoint<COLS>> Default for Dataset<COLS, Data> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const COLS: usize, DataElement: ToString> Dataset<COLS, [DataElement; COLS]> {
    /**
    Takes in a set of columns and creates a dataset from these.

    # Examples
    ```
    use delfi::Dataset;

    let t = [0, 1, 2, 3, 4, 5];
    let x = [2, 3, 5, 8, 12, 17];
    let _ = Dataset::from_columns([t, x]);
    ```
    */
    pub fn from_columns<IntoIter, Iter>(columns: [IntoIter; COLS]) -> Self
    where
        IntoIter: IntoIterator<Item = DataElement, IntoIter = Iter>,
        Iter: Iterator<Item = DataElement>,
    {
        let mut columns: [Iter; COLS] = columns
            .into_iter()
            .map(IntoIterator::into_iter)
            .collect::<Vec<Iter>>()
            .try_into()
            .map_err(|_| ())
            .expect("Failed to coerce vec into array");
        let mut data = Vec::new();
        'outer: loop {
            let mut temp = Vec::with_capacity(COLS);
            for col in columns.iter_mut() {
                if let Some(data) = col.next() {
                    temp.push(data);
                } else {
                    break 'outer;
                }
            }
            // map_err is required to avoid restricting Debug to be implemented for IntoIterator
            let row: [DataElement; COLS] = temp
                .try_into()
                .map_err(|_| ())
                .expect("Failed to coerce vec into array");
            data.push(row);
        }

        let labels = None;

        Dataset { labels, data }
    }
}

impl<const COLS: usize, Data: Datapoint<COLS>> Dataset<COLS, Data> {
    /**
    Saves a dataset to a given file. The filepath must be valid.
    Accepts anything path-like.

    # Examples
    ```
    # use delfi::Dataset;
    #
    # let t = [0, 1, 2, 3, 4, 5];
    # let x = [2, 3, 5, 8, 12, 17];
    # let dataset = Dataset::from_columns([t, x]);
    #
    dataset.save("./resources/data/examples/save-short.csv").unwrap();
    ```

    ```
    # use delfi::Dataset;
    #
    # let t = [0, 1, 2, 3, 4, 5];
    # let x = [2, 3, 5, 8, 12, 17];
    # let dataset = Dataset::from_columns([t, x]);
    #
    let directory = std::fs::canonicalize("./resources/data/examples/").unwrap();
    let filepath = directory.join("save-long.csv");
    dataset.save(&filepath).unwrap();
    ```

    */
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
    fn new() {
        let mut dataset = Dataset::new();
        assert_eq!(dataset.n_datapoints(), 0);
        dataset.push([1, 2, 3]);
        assert_eq!(dataset.n_datapoints(), 1);
        dataset.push([3, 4, 5]);
        assert_eq!(dataset.n_datapoints(), 2);
        assert_eq!(dataset.n_columns(), 3);
    }

    #[test]
    fn labels() {
        let x = [2, 3, 4];
        let y = [5, 6, 7];
        let mut dataset = Dataset::from_columns([x, y]);
        assert_eq!(dataset.get_labels(), None);
        dataset.set_labels(["x", "y"]);
        assert_eq!(
            dataset.get_labels(),
            Some(&[String::from("x"), String::from("y")])
        );
    }

    #[test]
    fn size() {
        let mut dataset = Dataset::new();
        dataset.push([1, 2, 3]);
        dataset.push([3, 4, 5]);
        assert_eq!(dataset.n_columns(), 3);
        assert_eq!(dataset.n_datapoints(), 2);
        assert_eq!(dataset.n_rows(), 2);
        dataset.set_labels(["a", "b", "c"]);
        assert_eq!(dataset.n_columns(), 3);
        assert_eq!(dataset.n_datapoints(), 2);
        assert_eq!(dataset.n_rows(), 3);
    }

    // Check constructors
    fn check_size<const COLS: usize, Data: Datapoint<COLS>>(dataset: Dataset<COLS, Data>) {
        assert_eq!(dataset.n_columns(), 2);
        assert_eq!(dataset.n_rows(), 3);
    }

    // Rows
    #[test]
    fn from_datapoints_array() {
        let array = [[1, 2], [3, 4], [5, 6]];
        let dataset = Dataset::from_datapoints(array);
        println!("{:?}", dataset);
        check_size(dataset);
    }

    #[test]
    fn from_datapoints_iterator() {
        let iterator = [[1, 2], [3, 4], [5, 6]].into_iter();
        let dataset = Dataset::from_datapoints(iterator);
        println!("{:?}", dataset);
        check_size(dataset);
    }

    #[test]
    fn from_datapoints_vec() {
        let vector = vec![[1, 2], [3, 4], [5, 6]];
        let dataset = Dataset::from_datapoints(vector);
        println!("{:?}", dataset);
        check_size(dataset);
    }

    // Columns
    #[test]
    fn from_columns_array() {
        let array = [[1, 3, 5], [2, 4, 6]];
        let dataset = Dataset::from_columns(array);
        println!("{:?}", dataset);
        check_size(dataset);
    }

    #[test]
    fn from_columns_iterator() {
        let iterator = [[1, 3, 5].into_iter(), [2, 4, 6].into_iter()];
        let dataset = Dataset::from_columns(iterator);
        println!("{:?}", dataset);
        check_size(dataset);
    }

    #[test]
    fn from_columns_vec() {
        let vector = [vec![1, 3, 5], vec![2, 4, 6]];
        let dataset = Dataset::from_columns(vector);
        println!("{:?}", dataset);
        check_size(dataset);
    }
}
