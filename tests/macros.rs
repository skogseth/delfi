#![cfg(feature = "macros")]

use std::path::PathBuf;

use delfi::{Datapoint, Dataset};

const ROOT: &str = "./resources/data/tests/macros";

#[derive(Debug, Datapoint)]
struct Count {
    ch: char,
    i: usize,
}

#[derive(Debug, Datapoint)]
struct Point(f64, f64);

#[test]
fn derive_named() {
    let a = Count { ch: 'a', i: 49 };
    let record = a.record();
    let compare = [('a').to_string(), (49).to_string()];
    assert_eq!(record, compare);
}

#[test]
fn derive_unnamed() {
    let p = Point(3.0, 4.0);
    let record = p.record();
    let compare = [(3.0).to_string(), (4.0).to_string()];
    assert_eq!(record, compare);
}

#[test]
fn trait_objects() {
    let a = Count { ch: 'a', i: 49 };
    let p = Point(3.0, 4.0);
    let list: [&dyn Datapoint<2>; 2] = [&a, &p];
    for dp in list {
        println!("{:?}", dp.record());
    }
}

#[test]
fn dataset_from_named() {
    #[derive(Datapoint)]
    struct MyDatapoint {
        c: char,
        i: usize,
    }

    let mut dataset = Dataset::new();

    let datapoint_1 = MyDatapoint { c: 'a', i: 100 };
    dataset.push(datapoint_1);

    let datapoint_2 = MyDatapoint { c: 'b', i: 200 };
    dataset.push(datapoint_2);

    let filepath = PathBuf::from(ROOT).join("dataset-named.csv");
    dataset.save(filepath).unwrap();
}

#[test]
fn dataset_from_unnamed() {
    #[derive(Datapoint)]
    struct MyDatapoint(String, usize, f64);

    let dp1 = MyDatapoint("Hello".to_owned(), 4, 10.2);
    let dp2 = MyDatapoint("World".to_owned(), 5, 3.14);
    let dataset = Dataset::from_datapoints([dp1, dp2]);

    let filepath = PathBuf::from(ROOT).join("dataset-unnamed.csv");
    dataset.save(filepath).unwrap();
}
