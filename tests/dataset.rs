use std::path::PathBuf;

use delfi::{dataset, Dataset};

const ROOT: &str = "./resources/data/tests/dataset";

#[test]
fn columns_using_arrays() {
    let x = [1, 2, 3];
    let y = [3, 4, 5];
    let dataset = Dataset::columns([x,y],["x","y"]);
    let filepath = PathBuf::from(ROOT).join("columns-arrays.csv");
    dataset.save(filepath).unwrap();
}

#[test]
fn macro_using_arrays() {
    let x = [1, 2, 3];
    let y = [3, 4, 5];
    let dataset = dataset!{
        "x" => x,
        "y" => y,
    };
    let filepath = PathBuf::from(ROOT).join("macro-arrays.csv");
    dataset.save(filepath).unwrap();
}

#[test]
fn columns_using_vectors() {
    let t = vec![0.0, 1.0, 2.0];
    let x = vec![0.5, 2.2, 6.3];
    let dataset = Dataset::columns([t,x], ["time", "length"]);
    let filepath = PathBuf::from(ROOT).join("columns-vectors.csv");
    dataset.save(filepath).unwrap();
}

#[test]
fn macro_using_vectors() {
    let t = vec![0.0, 1.0, 2.0];
    let x = vec![0.5, 2.2, 6.3];
    let dataset = dataset!{
        "time" => t,
        "length" => x,
    };
    let filepath = PathBuf::from(ROOT).join("macro-vectors.csv");
    dataset.save(filepath).unwrap();
}
