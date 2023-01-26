use std::path::PathBuf;

use delfi::{dataset, Dataset};
use ndarray::Array;

const ROOT: &str = "./resources/data/tests/ndarray";

#[test]
fn columns_using_arrays() {
    const N: usize = 1000;
    let t = Array::linspace(0., 10., N + 1);
    let x = Array::logspace(10., 0., 2., N + 1);

    let dataset = Dataset::from_columns([t, x]).with_labels(["time", "length"]);

    let filepath = PathBuf::from(ROOT).join("columns-arrays.csv");
    dataset.save(filepath).unwrap();
}

#[test]
fn macro_using_arrays() {
    const N: usize = 1000;
    let t = Array::linspace(0., 10., N + 1);
    let x = Array::logspace(10., 0., 2., N + 1);

    let dataset = dataset! {
        "time" => t,
        "length" => x,
    };

    let filepath = PathBuf::from(ROOT).join("macro-arrays.csv");
    dataset.save(filepath).unwrap();
}
