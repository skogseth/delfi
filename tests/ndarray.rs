use delfi::{dataset, Dataset};
use ndarray::Array;

#[test]
fn columns_using_arrays() {
    const N: usize = 1000;
    let t = Array::linspace(0., 10., N+1);
    let x = Array::logspace(10., 0., 2., N+1);
    
    let _dataset = Dataset::columns([t,x], ["time", "length"]);
    
    //dataset.save("./resources/data/tests/columns-vectors.csv").unwrap();
}

#[test]
fn macro_using_arrays() {
    const N: usize = 1000;
    let t = Array::linspace(0., 10., N+1);
    let x = Array::logspace(10., 0., 2., N+1);
        
    let _dataset = dataset!{
        "time" => t,
        "length" => x,
    };

    //dataset.save("./resources/data/tests/macro-vectors.csv").unwrap();
}